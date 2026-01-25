use chrono::{DateTime, Duration as ChronoDuration, Local, LocalResult, NaiveTime, TimeZone};
use clap::Parser;
use dotenv::dotenv;
use fantoccini::{ClientBuilder, Locator};
use scraper::Html;
use std::fs::File;
use std::io::Write;
use tokio::process::Command;
use tokio::time::{sleep, Duration};

// mod claude;

const SLEEP_TIME: u64 = 3;
const OLD_REDDIT: &str = "https://old.reddit.com";
const ITEM_LEN: usize = 15;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// Optional date override, format: YYYY-MM-DD
    #[arg(value_name = "DATE")]
    date: Option<String>,

    /// Run daily at the configured clock time
    #[arg(long)]
    daily: bool,

    /// Time of day to run in daily mode, format: HH:MM (24-hour)
    #[arg(long, value_name = "CLOCK", default_value = "02:00", value_parser = parse_clock)]
    clock: NaiveTime,
}

fn parse_clock(value: &str) -> Result<NaiveTime, String> {
    NaiveTime::parse_from_str(value, "%H:%M")
        .map_err(|_| "clock must be in HH:MM 24-hour format".to_string())
}

fn local_datetime(
    date: chrono::NaiveDate,
    time: NaiveTime,
    fallback: DateTime<Local>,
) -> DateTime<Local> {
    let naive = date.and_time(time);
    match Local.from_local_datetime(&naive) {
        LocalResult::Single(dt) => dt,
        LocalResult::Ambiguous(dt, _) => dt,
        LocalResult::None => {
            log::warn!("local time {naive} is invalid; using fallback {fallback}");
            fallback
        }
    }
}

fn next_run_at(clock: NaiveTime, now: DateTime<Local>) -> DateTime<Local> {
    let today = now.date_naive();
    let mut next = local_datetime(today, clock, now + ChronoDuration::hours(1));
    if next < now {
        let tomorrow = today + ChronoDuration::days(1);
        next = local_datetime(tomorrow, clock, now + ChronoDuration::days(1));
    }
    next
}

fn sanitize_html(html: &str) -> String {
    let fragment = Html::parse_fragment(html);
    fragment
        .root_element()
        .text()
        .collect::<Vec<_>>()
        .join("   ")
}

fn extract_content(html: &str) -> String {
    use dom_content_extraction::{scraper::Html as IHtml, DensityTree};

    let document = IHtml::parse_document(&html);
    let mut dtree = match DensityTree::from_document(&document) {
        Ok(tree) => tree,
        Err(err) => {
            log::warn!("density tree init failed: {}", err);
            return sanitize_html(html);
        }
    };
    // let sorted_nodes = dtree.sorted_nodes();
    // let node_id = sorted_nodes.last().unwrap().node_id;

    // println!("{}", get_node_text(node_id, &document));

    if let Err(err) = dtree.calculate_density_sum() {
        log::warn!("density sum failed: {}", err);
        return sanitize_html(html);
    }
    let extracted_content = match dtree.extract_content(&document) {
        Ok(content) => content,
        Err(err) => {
            log::warn!("content extract failed: {}", err);
            return sanitize_html(html);
        }
    };

    // println!("extracted_content {}", extracted_content);

    extracted_content
}

async fn run_call_claude(formatted_date: &str) -> Result<(), Box<dyn std::error::Error>> {
    let exe_suffix = std::env::consts::EXE_EXTENSION;
    let exe_name = if exe_suffix.is_empty() {
        "call_claude".to_string()
    } else {
        format!("call_claude.{exe_suffix}")
    };

    let call_claude_path = std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|dir| dir.join(&exe_name)))
        .filter(|candidate| candidate.exists());

    let status = if let Some(path) = call_claude_path {
        Command::new(path).arg(formatted_date).status().await?
    } else {
        log::info!("call_claude binary missing; running via cargo");
        Command::new("cargo")
            .args(["run", "--bin", "call_claude", "--", formatted_date])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .status()
            .await?
    };

    if !status.success() {
        return Err(format!("call_claude exited with status {status}").into());
    }

    Ok(())
}

fn today_string() -> String {
    Local::now().date_naive().format("%Y-%m-%d").to_string()
}

async fn run_pipeline(formatted_date: &str) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("date: {formatted_date}");
    // Set up the WebDriver client
    let c = ClientBuilder::rustls()?
        .connect("http://localhost:4444")
        .await?;

    // Navigate to the Rust subreddit
    c.goto(&format!("{OLD_REDDIT}/r/rust/")).await?;

    // Find all post titles
    let posts = c.find_all(Locator::Css("#siteTable a.title")).await?;

    let mut post_results: Vec<(String, String)> = vec![];
    for post in posts.iter().skip(2).take(ITEM_LEN) {
        let title = post.text().await?;
        let href = post.attr("href").await?.expect("should be a url");
        log::info!("=> {}, {}", title, href);
        // let html = post.html(false).await?;
        // println!("Post html: {:?}", html);
        // println!("--------------------------------");
        post_results.push((title, href));
    }

    log::info!("Post Results: {:?}", post_results.len());

    let mut target_texts: Vec<(String, String)> = vec![];
    let mut jump_links: Vec<String> = vec![];
    for post in post_results {
        let (_, href) = post;
        if !href.starts_with("https://alb.reddit.com/") {
            if href.starts_with("/") {
                // relative url in reddit site
                let url = OLD_REDDIT.to_string() + &href;
                c.goto(&url).await?;

                let first_content = c.find(Locator::Css("div.entry .usertext-body")).await?;
                let html_content = first_content.html(true).await?;
                let sanitized_text = sanitize_html(&html_content);
                // println!("Sanitized post text: {}", sanitized_text);
                target_texts.push((url, sanitized_text));

                let links_in_content = first_content.find_all(Locator::Css("a")).await?;
                for link in links_in_content {
                    let mut url = link.attr("href").await?.expect("should be a url");
                    log::info!("link in reddit post: {}", url);
                    if url.starts_with("/") {
                        url = format!("{OLD_REDDIT}{url}")
                    }
                    if !url.starts_with("mailto") {
                        jump_links.push(url);
                    }
                }

                sleep(Duration::from_secs(SLEEP_TIME)).await;
            } else {
                // outsite url
                let url = href;
                if let Ok(_) = c.goto(&url).await {
                    if let Ok(body) = c.find(Locator::Css("body")).await {
                        let html = body.html(false).await?;
                        let extracted_text = extract_content(&html);
                        // let extracted_text = sanitize_html(&html);
                        // println!("extracted text: {}", extracted_text);
                        target_texts.push((url, extracted_text));
                    }
                }
            }
        }
    }

    log::info!("========");
    log::info!("jump links len: {}", jump_links.len());
    // fetch succeding links
    for link in jump_links {
        log::info!("link: {}", link);
        if let Ok(_) = c.goto(&link).await {
            if let Ok(body) = c.find(Locator::Css("body")).await {
                let html = body.html(false).await?;
                let extracted_text = extract_content(&html);
                // let extracted_text = sanitize_html(&html);
                // println!("extracted text: {}", extracted_text);
                target_texts.push((link, extracted_text));
            }
        }
    }

    // Close the browser
    c.close().await?;

    let filename = format!("tmp/rust_diary-{formatted_date}.txt");
    // write to file
    let mut file = File::create(&filename)?;

    target_texts.reverse();
    let content = target_texts
        .iter()
        .map(|(url, text)| format!("{url}\n-->>-->>\n{text}"))
        .collect::<Vec<String>>()
        .join("\n======>\n");

    // Write the data to the file
    file.write_all(content.as_bytes())?;

    // Optionally, flush the file to ensure all data is written
    file.flush()?;

    run_call_claude(&formatted_date).await?;

    log::info!("Task finished");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv().ok();

    let args = Args::parse();

    if args.daily {
        if args.date.is_some() {
            log::warn!("date override ignored in daily mode");
        }
        loop {
            let now = Local::now();
            let next = next_run_at(args.clock, now);
            let wait = (next - now).to_std().unwrap_or_else(|_| Duration::from_secs(0));
            log::info!("next run scheduled at {}", next);
            sleep(wait).await;

            let formatted_date = today_string();
            if let Err(err) = run_pipeline(&formatted_date).await {
                log::error!("daily run failed: {err}");
            }
        }
    } else {
        let formatted_date = args.date.unwrap_or_else(today_string);
        run_pipeline(&formatted_date).await?;
    }

    Ok(())
}
