#![feature(str_strip)]
use reqwest;
use std::error::Error;

pub use article::{Article, ArticleErr, CollectionErr, Collector};
pub use config::{
    Config, ConfigErr, REFERENCE_PREFIX, WIKI_ARTICLE_PREFIX, WIKI_ARTICLE_PREFIX_BLACKLIST,
    WIKI_ARTICLE_SUFFIX_BLACKLIST, WIKI_DOMAIN,
};
pub use url::{URLErr, URL};

pub mod article;
pub mod config;
pub mod url;

/// The main function of this library. Running this allows you to find a
/// graph around a certain set of Wikipedia articles and possibly the shortest
/// paths between them.
pub async fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    // TODO: Implement running logic.
    println!("Creating collector");
    let mut collector = Collector::new();
    println!("Iterating URLs");
    for url in cfg.iter_urls() {
        println!("Current URL: {}", url.to_string());
        let resp = collector.get(url).await;
        println!("{:?}", resp);
    }
    Ok(())
}
