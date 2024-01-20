use crate::validation::{is_valid_url, ValidationError};
use scraper::{Html, Selector};

pub struct Article {
    url: String,
    title: Option<String>,
    content: Option<String>,
}

impl Article {
    async fn new(url: String) -> Result<Article, ValidationError> {
        let mut article = Article {
            url,
            title: None,
            content: None,
        };

        let fetched_article = fetch_url(&article.url).await?;
        let (title, content) = parse_html(&fetched_article)?;
        article.title = Some(title);
        article.content = Some(content);

        return Ok(article);
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

fn parse_html(html: &str) -> Result<(String, String), ValidationError> {
    let document = Html::parse_document(html);

    let h1_selector = Selector::parse("h1").map_err(|_| ValidationError::TitleNotFound)?;
    let h1 = document
        .select(&h1_selector)
        .next()
        .ok_or(ValidationError::TitleNotFound)?;
    let h1_content = h1.text().collect::<String>();

    let article_selector =
        Selector::parse("article").map_err(|_| ValidationError::ArticleNotFound)?;
    let article = document
        .select(&article_selector)
        .next()
        .ok_or(ValidationError::ArticleNotFound)?;
    let article_content = article.text().collect::<String>();

    Ok((h1_content, article_content))
}

async fn fetch_url(url: &str) -> Result<String, ValidationError> {
    println!("Fetching URL {} in progress...", url);
    let response = reqwest::get(url).await;

    match response {
        Ok(res) => {
            let url_body = res.text().await;
            match url_body {
                Ok(html) => {
                    println!("✅ Article fetched!");
                    Ok(html)
                }
                Err(_) => Err(ValidationError::UnreachableResource),
            }
        }
        Err(_) => Err(ValidationError::UnreachableResource),
    }
}

pub async fn get_article(url: String) -> Result<Article, ValidationError> {
    if is_valid_url(&url) {
        let article = Article::new(url).await?;

        Ok(article)
    } else {
        Err(ValidationError::InvalidUrlFormat)
    }
}
