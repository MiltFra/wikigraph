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

#[derive(Error, Debug)]
pub enum ArticleErr {
  #[error("Missing prefix.")]
  MissingPrefix,
  #[error("Blacklisted article prefix found. ({0})")]
  BlacklistedPrefix(String),
}

pub fn validate_wikipedia_article(url: &str) -> Result<(), Box<dyn Error>> {
  if !url.starts_with(WIKI_ARTICLE_PREFIX) {
    return Err(Box::new(ArticleErr::MissingPrefix));
  }
  let url = url.trim_start_matches(WIKI_ARTICLE_PREFIX);
  for blacklisted in WIKI_ARTICLE_BLACKLIST.iter() {
    if url.starts_with(blacklisted) {
      return Err(Box::new(ArticleErr::BlacklistedPrefix(String::from(
        *blacklisted,
      ))));
    }
  }
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn is_wikipedia_article_valid() -> Result<(), Box<dyn Error>> {
    validate_wikipedia_article("https://en.wikipedia.org/wiki/Wikipedia")?;
    validate_wikipedia_article("https://en.wikipedia.org/wiki/Help!_(film)")?;
    Ok(())
  }

  #[test]
  fn is_wikipedia_article_invalid() {
    if let Ok(_) = validate_wikipedia_article("https://en.wikipedia.org/wiki/Help:Contents") {
      panic!("Test1 failed.");
    }
    if let Ok(_) = validate_wikipedia_article("https://en.wikipedia.org/wiki/Wikipedia:Contact_us")
    {
      panic!("Test 2 failed.")
    }
  }
}
