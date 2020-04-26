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
    let mut collector = Collector::new();
    for x in cfg.urls.iter() {
        for y in cfg.urls.iter() {
            if *x == *y {
                continue;
            }
            let path: Vec<_> = collector
                .get_path(x, y)
                .await?
                .into_iter()
                .map(|x| x.get_url().get_name()).collect();
            eprintln!(
                "Found path from {} to {} of length {}",
                x.get_name(),
                y.get_name(),
                path.len()
            );
            println!("{:?}", path);
        }
    }
    Ok(())
}
