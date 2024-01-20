use reqwest;
use scraper::{Html, Selector};
use std::env::args;
use std::fmt;
use tokio;
use url::Url;

enum ValidationError {
    InvalidUrlFormat,    // Not a valid URL
    UnreachableResource, // Error status from get request
    ArticleNotFound,     // <article> not found
    TitleNotFound,       // <h1> not found
}

impl fmt::Debug for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Custom formatting logic here
        match self {
            ValidationError::InvalidUrlFormat => write!(f, "Invalid URL Format"),
            ValidationError::UnreachableResource => write!(f, "Unreachable Resource"),
            ValidationError::TitleNotFound => write!(f, "Title Not Found"),
            ValidationError::ArticleNotFound => write!(f, "Article Not Found"),
        }
    }
}

fn is_valid_url(url: &str) -> bool {
    let result = Url::parse(url);
    result.is_ok() // Without ; at end, the value is returned automatically. It's a shorthand for return result.is_ok();
}

struct Article {
    url: String,
    title: Option<String>,
    content: Option<String>,
}

impl Article {
    fn new(url: String) -> Article {
        Article {
            url: url,
            title: None,
            content: None,
        }
    }
}

trait Describable {
    fn describe(&self) -> String;
}

impl Describable for Article {
    fn describe(&self) -> String {
        match &self.title {
            Some(title) => {
                format!("{}", title)
            }
            None => {
                format!("No title found!")
            }
        }
    }
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

fn get_article(url: String) -> Result<Article, ValidationError> {
    if is_valid_url(&url) {
        let article = Article::new(url);

        Ok(article)
    } else {
        Err(ValidationError::InvalidUrlFormat)
    }
}

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
