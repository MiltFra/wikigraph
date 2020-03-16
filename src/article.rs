use super::*;
use futures;
use regex::Regex;
use reqwest;
use std::collections::HashSet;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::sync::mpsc;
use thiserror::Error;

/// A struct representing a Wikipedia article with attributes like
/// the URL, related articles and eventually more.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Article {
    /// URL of the article; where you'd find it in your web browser.
    url: URL,
    /// All the URLs of other articles that are referenced within the article.
    references: HashSet<URL>,
}

/// ArticleErr is an enum that contains possible error values that
/// could occur during the creation of a new Article in Article::new.
///
/// Keep in mind that this includes a lot of I/O operation.
#[derive(Error, Debug)]
pub enum ArticleErr {
    #[error("Line ended while parsing URL")]
    UnexpectedEOL,
}

impl Article {
    pub fn new(url: URL) -> Self {
        Article {
            url: url,
            references: HashSet::new(),
        }
    }
    pub fn parse(url: URL, site: String) -> Result<Self, Box<dyn Error>> {
        println!("Parsing...");
        let mut refs = HashSet::new();
        let lines = site.lines();
        for mut line in lines {
            while !line.is_empty() {
                if line.starts_with("<a href=\"/wiki/") {
                    line = line.strip_prefix(REFERENCE_PREFIX).unwrap_or("");
                    let end;
                    match line.find('"') {
                        Some(i) => end = i,
                        None => {
                            println!("{}", line);
                            return Err(Box::new(ArticleErr::UnexpectedEOL));
                        }
                    }
                    if let Ok(ref_url) = URL::new(&line[..end]) {
                        refs.insert(ref_url);
                    }
                    line = &line[end..];
                    continue;
                }
                // Strip one character from the left.
                line = line
                    .chars()
                    .next()
                    .map(|c| &line[c.len_utf8()..])
                    .unwrap_or("");
            }
        }
        let mut v: Vec<String> = refs.iter().map(|x| x.to_string()).collect();
        v.sort();
        v.iter().for_each(|x| println!("{}", x));
        Ok(Article {
            url: url,
            references: refs,
        })
    }
}

pub struct Collector {
    cache: HashMap<URL, Article>,
    processed: i32,
    work_queue: VecDeque<URL>,
    client: reqwest::Client,
}
#[derive(Error, Debug)]
pub enum CollectionErr {
    #[error("HTTP request failed.")]
    RequestError,
}

impl Collector {
    pub fn new() -> Self {
        Collector {
            cache: HashMap::new(),
            processed: 0,
            work_queue: VecDeque::new(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn get(&mut self, url: &URL) -> Result<Article, Box<dyn Error>> {
        self.processed += 1;
        if let Some(a) = self.cache.get(url) {
            return Ok(a.clone());
        }
        let r = self.client.get(&url.to_string()).send().await?;
        Ok(Article::parse(url.clone(), r.text().await?)?)
    }
}
