use chrono::Local;
use dotenv::dotenv;
use std::fs::File;
use std::io::{Read, Write};

mod claude;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv().ok();

    let api_key =
        std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY must be set in .env file");

    let formatted_date = if let Some(date) = std::env::args().nth(1) {
        log::info!("date: {date}");
        date
    } else {
        // write to tmp file
        let today = Local::now().date_naive();
        // Format the date in the form "YYYY-MM-DD"
        let formatted_date = today.format("%Y-%m-%d").to_string();
        formatted_date
    };

    let filename = format!("tmp/rust_diary-{formatted_date}.txt");
    let mut file = File::open(&filename)?;
    // Create a string to hold the file contents
    let mut tmpl_contents = String::new();

    // Read the entire file into the string
    file.read_to_string(&mut tmpl_contents)?;
    let target_texts: Vec<&str> = tmpl_contents.split("======>").collect();

    log::info!("========");
    log::info!("target texts len: {}", target_texts.len());
    let mut outputs: Vec<(String, String)> = vec![];
    // call claude API to summarize these contents
    for mut text in target_texts {
        log::info!("\n============>>>");
        log::info!("\n{:?}", text);
        let url_content = text.split("-->>-->>").collect::<Vec<&str>>();
        let url = &url_content[0];
        // let content = &url_content[1];

        if text.len() > 5000 {
            text = &text[..5000]
        }
        if text.len() >= 300 {
            let summarization = claude::call_claude_to_summarize(&api_key, &text).await?;
            log::info!("\nUrl: {}\nOutput: {}", url, summarization);
            outputs.push((url.to_string(), summarization));
        }
    }

    let item_tmpl = r#"
### TITLE

$CONTENT$

[$URL$]($URL$)
    
"#;

    let content = outputs
        .iter()
        .map(|(url, text)| item_tmpl.replace("$CONTENT$", text).replace("$URL$", url))
        .collect::<Vec<String>>()
        .join("\n");

    // read tmpl
    let mut file = File::open("rust_diary_tmpl.md")?;

    // Create a string to hold the file contents
    let mut tmpl_contents = String::new();

    // Read the entire file into the string
    file.read_to_string(&mut tmpl_contents)?;
    let output_content = tmpl_contents
        .replace("$DATE$", &formatted_date)
        .replace("$CONTENT$", &content);

    // output markdown files
    let filename = format!("outputs/rust_diary-{formatted_date}.md");
    let mut file = File::create(&filename)?;
    // Write the data to the file
    file.write_all(output_content.as_bytes())?;

    // Optionally, flush the file to ensure all data is written
    file.flush()?;

    Ok(())
}
