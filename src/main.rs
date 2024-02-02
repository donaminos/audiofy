mod api;
mod article;
mod validation;

use article::{get_article, Describable};
use reqwest;
use std::env;
use tokio;
use api::openai_client::OpenAIClient;

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
                let article_title = article.describe();
                println!("⏩ Title: {}", article_title);
                println!("🎤 Audiofy...");

                let client = OpenAIClient::new();

                let output_path = format!("output/{}.mp3", article_title);
                // For testing purpose: we send the title only at the moment
                client.text_to_speech(article_title, &output_path).await;
            }
            Err(e) => {
                print!("❌ Failed to process argument at index {}: {:?}", index, e);
            }
        }
    }

    Ok(())
}
