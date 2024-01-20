use std::fmt;
use url::Url;
pub enum ValidationError {
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

pub fn is_valid_url(url: &str) -> bool {
    let result = Url::parse(url);
    result.is_ok() // Without ; at end, the value is returned automatically. It's a shorthand for return result.is_ok();
}
