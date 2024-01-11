use reqwest;
use std::env::args;
use tokio;
use url::Url;
use scraper::{Html, Selector};

fn is_valid_url(url: &str) -> bool {
    let result = Url::parse(url);
    result.is_ok() // Without ; at end, the value is returned automatically. It's a shorthand for return result.is_ok();
}

fn parse_html(html: &str) {
    // Parse the HTML
    let document = Html::parse_document(html);
    // Create a CSS selector
    let h1_selector = Selector::parse("h1").unwrap();
    // Get the h1 node content
    let h1 = document.select(&h1_selector).next().unwrap();
    let text = h1.text().collect::<Vec<_>>();

    println!("{:?}", text);
}

async fn fetch_url(url: &str) {
    println!("Fetching URL {} in progress...", url);
    let response = reqwest::get(url).await;

    match response {
        Ok(res) => {
            let url_body = res.text().await;
            match url_body {
                Ok(html) => {
                    println!("✅ Succeeded!");
                    parse_html(&html);
                }
                Err(e) => {
                    println!("❌ Failed to get URL body: {}", e.to_string())
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to fetch URL: {}", e.to_string());
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = args().collect();
    println!("Audiofy: Transform your favorites articles to a podcast 🚀");

    for (index, arg) in args.iter().skip(1).enumerate() {
        if is_valid_url(arg) {
            fetch_url(arg).await;
        } else {
            println!("- Invalid argument at index {}: {}", index, arg);
        }
    }

    Ok(())
}