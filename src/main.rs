use chrono::Local;
use dotenv::dotenv;
use fantoccini::{ClientBuilder, Locator};
use scraper::Html;
use std::fs::File;
use std::io::Write;
use tokio::time::{sleep, Duration};

// mod claude;

const SLEEP_TIME: u64 = 1;
const OLD_REDDIT: &str = "https://old.reddit.com";
const ITEM_LEN: usize = 15;

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
    let mut dtree = DensityTree::from_document(&document); // &scraper::Html
                                                           // let sorted_nodes = dtree.sorted_nodes();
                                                           // let node_id = sorted_nodes.last().unwrap().node_id;

    // println!("{}", get_node_text(node_id, &document));

    dtree.calculate_density_sum();
    let extracted_content = dtree.extract_content(&document);

    // println!("extracted_content {}", extracted_content);

    extracted_content
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Set up the WebDriver client
    let c = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await?;

    // Navigate to the Rust subreddit
    c.goto(&format!("{OLD_REDDIT}/r/rust/")).await?;

    // Find all post titles
    let posts = c.find_all(Locator::Css("#siteTable a.title")).await?;

    let mut post_results: Vec<(String, String)> = vec![];
    for post in &posts[2..(ITEM_LEN + 2)] {
        let title = post.text().await?;
        let href = post.attr("href").await?.expect("should be a url");
        println!("=> {}, {}", title, href);
        // let html = post.html(false).await?;
        // println!("Post html: {:?}", html);
        // println!("--------------------------------");
        post_results.push((title, href));
    }

    println!("Post Results: {:?}", post_results.len());

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
                    println!("link in reddit post: {}", url);
                    if url.starts_with("/") {
                        url = format!("{OLD_REDDIT}{url}")
                    }
                    jump_links.push(url);
                }

                sleep(Duration::from_secs(SLEEP_TIME)).await;
            } else {
                // outsite url
                let url = href;
                c.goto(&url).await?;
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

    println!("========");
    println!("jump links len: {}", jump_links.len());
    // fetch succeding links
    for link in jump_links {
        println!("link: {}", link);
        c.goto(&link).await?;
        if let Ok(body) = c.find(Locator::Css("body")).await {
            let html = body.html(false).await?;
            let extracted_text = extract_content(&html);
            // let extracted_text = sanitize_html(&html);
            // println!("extracted text: {}", extracted_text);
            target_texts.push((link, extracted_text));
        }
    }

    // Close the browser
    c.close().await?;

    // write to tmp file
    let today = Local::now().date_naive();
    // Format the date in the form "YYYY-MM-DD"
    let formatted_date = today.format("%Y-%m-%d").to_string();

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

    Ok(())
}
