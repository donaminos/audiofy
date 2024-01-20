mod article;
mod validation;

use article::{get_article, Describable};
use reqwest;
use std::env::args;
use tokio;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = args().collect();
    println!("Audiofy: Transform your favorites articles to a podcast 🚀");

    for (index, arg) in args.iter().skip(1).enumerate() {
        let url = arg.to_string();
        match get_article(url) {
            Ok(article) => {
                println!("Article created");
            }
            Err(e) => {
                print!("Error {:?}", e);
            }
        }
    }

    Ok(())
}
