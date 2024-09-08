use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;

pub async fn call_claude_to_summarize(api_key: &str, text: &str) -> Result<String, Box<dyn Error>> {
    // Replace with your actual API key
    let client = Client::new();

    // The content you want to summarize
    let content_to_summarize = text;
    // Construct the prompt
    let prompt = format!(
        "请用中文总结以下内容:\n\n{}\n\nSummary:",
        content_to_summarize
    );

    // Make the API request
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("Content-Type", "application/json")
        .header("anthropic-version", "2023-06-01")
        .header("x-api-key", api_key)
        .json(&json!({
            "model": "claude-3-sonnet-20240229",
            "max_tokens": 1024,
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ]
        }))
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        let body: Value = response.json().await?;
        if let Some(content) = body["content"][0]["text"].as_str() {
            // println!("Summary: {}", content);
            return Ok(content.to_string())
        } else {
            println!("Couldn't extract summary from the response");
        }
    } else {
        println!("Error: {:?}", response.text().await?);
    }

    Ok("".to_string())
}
