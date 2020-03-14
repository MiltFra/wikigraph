use super::*;
use std::error::Error;
use thiserror::Error;

/// A struct representing a Wikipedia article with attributes like
/// the URL, related articles and eventually more.
pub struct Article {
    /// URL of the article; where you'd find it in your web browser.
    url: URL,
    /// All the URLs of other articles that are referenced within the article.
    references: Vec<URL>,
}

/// ArticleErr is an enum that contains possible error values that
/// could occur during the creation of a new Article in Article::new.
///
/// Keep in mind that this includes a lot of I/O operation.
pub enum ArticleErr {}

impl Article {
    pub fn new(url: URL) -> Self {
        Article {
            url: url,
            references: Vec::new(),
        }
    }
}
