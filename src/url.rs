use super::*;
use std::error::Error;
use thiserror::Error;

/// Contains the prefix that is used to identify Wikipedia articles.
///
/// Any wikipedia article must therefore be of the form
/// "<WIKI_ARTICLE_PREFIX><ARTICLE_NAME>".
pub const WIKI_ARTICLE_PREFIX: &str = "https://en.wikipedia.org/wiki/";

/// Contains prefixes of webpages that are not considered Wikipedia articles.
///
/// Any url of the form "<WIKI_ARTICLE_PREFIX><BLACKLIST_ELEMENT><REST>",
/// where BLACKLIST_ELEMENT is one of the elements in this array and
/// REST is the possibly empty rest of the string, is therefore invalid.
pub const WIKI_ARTICLE_BLACKLIST: [&str; 4] = ["Main_Page", "Help:", "Wikipedia:", "Special:"];

/// Contains possible errors that may occur when trying to create a URL.
#[derive(Error, Debug)]
pub enum URLErr {
    #[error("Missing prefix.")]
    MissingPrefix,
    #[error("Blacklisted article prefix found. ({0})")]
    BlacklistedPrefix(String),
}

/// An alias for String representing a URL to a valid Wikipedia article.
pub struct URL(String);

impl URL {
    pub fn new(url: &str) -> Result<URL, Box<dyn Error>> {
        URL::validate(url)?;
        Ok(URL(String::from(url)))
    }

    // TODO: Use closures and iterators to make this more functional and idiomatic.
    pub fn new_list(lines: std::str::Lines) -> Vec<URL> {
        let mut valid_urls = Vec::new();
        for addr in lines {
            let addr = addr.trim().trim_end_matches('\n');
            let res = URL::new(addr);
            match res {
                Err(e) => {
                    eprintln!("Found invalid URL ({}): {}", addr, e);
                    continue;
                }
                Ok(url) => valid_urls.push(url),
            }
        }
        valid_urls
    }

    fn validate(url: &str) -> Result<(), Box<dyn Error>> {
        if !url.starts_with(WIKI_ARTICLE_PREFIX) {
            return Err(Box::new(URLErr::MissingPrefix));
        }
        let url = url.trim_start_matches(WIKI_ARTICLE_PREFIX);
        for blacklisted in WIKI_ARTICLE_BLACKLIST.iter() {
            if url.starts_with(blacklisted) {
                return Err(Box::new(URLErr::BlacklistedPrefix(String::from(
                    *blacklisted,
                ))));
            }
        }
        Ok(())
    }

    pub fn to_string(&self) -> String {
        return self.0.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_wikipedia_article_valid() -> Result<(), Box<dyn Error>> {
        URL::validate("https://en.wikipedia.org/wiki/Wikipedia")?;
        URL::validate("https://en.wikipedia.org/wiki/Help!_(film)")?;
        Ok(())
    }

    #[test]
    fn is_wikipedia_article_invalid() {
        if let Ok(_) = URL::validate("https://en.wikipedia.org/wiki/Help:Contents") {
            panic!("Test1 failed.");
        }
        if let Ok(_) = URL::validate("https://en.wikipedia.org/wiki/Wikipedia:Contact_us") {
            panic!("Test 2 failed.")
        }
    }
}
