use super::*;
use reqwest;
use std::error::Error;
use thiserror::Error;

/// A struct representing a Wikipedia article with attributes like
/// the URL, the Head Line, related articles, etc.
pub struct Article {}

impl Article {
    pub async fn get(client: reqwest::Client, url: url::URL) -> Result<Article, Box<dyn Error>> {
        let body = client.get(&url.to_string()).send().await?;
        // TODO: Read about concurrency and implement proper ascynchronous requests.
        Ok(Article {})
    }

    // TODO: Use closures and iterators to make this more functional and idiomatic.
    pub fn get_list(
        client: reqwest::Client,
        urls: Vec<url::URL>,
    ) -> Result<Vec<Article>, Box<dyn Error>> {
        Ok(Vec::new())    
    }
}