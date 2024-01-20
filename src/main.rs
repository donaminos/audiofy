mod article;
mod validation;

use article::{get_article, Describable};
use reqwest;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let app_title = r#"
    ************************************************************************
    *                                                                      *
    *      Audiofy: Transform Your Favorite Articles into a Podcast 🚀     *
    *                                                                      *
    ************************************************************************
   "#;
    println!("{}", app_title);

    let args: Vec<String> = env::args().collect();
    // 1 and not O because the first arg is the path to the file
    if args.len() <= 1 {
        println!("🚫 No arguments were supplied!");
    }

    for (index, arg) in args.iter().skip(1).enumerate() {
        let url = arg.to_string();
        match get_article(url).await {
            Ok(article) => {
                println!("✅ Article fetched!");
                println!("⏩ Title: {}", article.describe());
                println!("🎤 Audiofy...");
            }
            Err(e) => {
                print!("❌ Failed to process argument at index {}: {:?}", index, e);
            }
        }
    }

    Ok(())
}
