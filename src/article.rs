use crate::validation::{is_valid_url, ValidationError};
use scraper::{Html, Selector};

pub struct Article {
    url: String,
    title: Option<String>,
    content: Option<String>,
}

impl Article {
    pub fn new(url: String) -> Article {
        Article {
            url: url,
            title: None,
            content: None,
        }
    }
}

pub trait Describable {
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

pub fn get_article(url: String) -> Result<Article, ValidationError> {
    if is_valid_url(&url) {
        let article = Article::new(url);

        Ok(article)
    } else {
        Err(ValidationError::InvalidUrlFormat)
    }
}
