use reqwest;
use std::env::args;
use tokio;
use url::Url;

fn is_valid_url(url: &str) -> bool {
    let result = Url::parse(url);
    result.is_ok() // Without ; at end, the value is returned automatically. It's a shorthand for return result.is_ok();
}

async fn fetch_url(url: &str) {
    println!("Fetching URL {} in progress...", url);
    let response = reqwest::get(url).await;

    match response {
        Ok(res) => {
            println!("✅ Succeeded!");

            let url_content = res.text().await;
            print!("{:?}",   url_content);
        }
        Err(e) => {
            println!("❌ Failed: {}",e.to_string() );
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