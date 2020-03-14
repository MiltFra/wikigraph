use super::*;
use std::error::Error;
use std::str;
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
#[derive(Debug)]
pub struct URL(String);

impl URL {
    /// The constructor checks whether a given string is actually a valid URL
    /// to a Wikipedia article and then converts this string into a new URL struct
    /// containing a clone of the string without the `WIKI_ARTICLE_PREFIX`.
    ///
    /// # Examples
    ///
    /// ```
    /// use wglib::URL;
    ///
    /// let myUrl = URL::new("https://en.wikipedia.org/wiki/Wikipedia").unwrap();
    /// let myUrl = URL::new("https://de.wikipedia.org/wiki/Wikipedia").unwrap_err();
    /// let myUrl = URL::new("https://en.wikipedia.org/wiki/Wikipedia:Contact_us").unwrap_err();
    /// ```
    pub fn new(url: &str) -> Result<Self, Box<dyn Error>> {
        Ok(URL(String::from(URL::extract_body(url)?)))
    }

    /// Given an iterator over possibly valid URLs of Wikipedia articles this function
    /// returns precisely those that are valid as URL structs.
    ///
    /// This process is stable, i.e. it preserves the input order.
    ///
    /// # Examples
    ///
    /// ```
    /// use wglib::URL;
    /// let contents = String::from(
    ///     "https://en.wikipedia.org/wiki/Wikipedia\n\
    ///     https://de.wikipedia.org/wiki/Wikipedia\n\
    ///     https://en.wikipedia.org/wiki/Wikipedia:Contact_us"
    /// );
    ///
    /// let my_list = URL::new_list(&contents);
    ///
    /// assert_eq!(my_list.len(), 1);
    /// assert_eq!(my_list[0].to_string(), "https://en.wikipedia.org/wiki/Wikipedia");
    /// ```
    pub fn new_list(contents: &String) -> Vec<URL> {
        contents.lines().filter_map(|x| URL::new(x).ok()).collect()
    }

    /// Validates that a given string does actually correspond to a valid Wikipedia
    /// article. Here we're only considering proper articles, not meta sites like
    /// the homepage.
    ///
    /// Then the body (the part after `WIKI_ARTICLE_PREFIX`) is returned.
    fn extract_body(url: &str) -> Result<&str, Box<dyn Error>> {
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
        Ok(url)
    }

    /// Reverts the actions of `URL::new()`. We get the `String` that is
    /// contained within the `URL` struct back. At least a clone of it.
    ///
    /// # Examples
    ///
    /// ```
    /// use wglib::URL;
    ///
    /// let myUrl = URL::new("https://en.wikipedia.org/wiki/Help!_(film)").unwrap();
    ///
    /// assert_eq!(myUrl.to_string(), "https://en.wikipedia.org/wiki/Help!_(film)");
    /// ```
    pub fn to_string(&self) -> String {
        format!("{}{}", WIKI_ARTICLE_PREFIX, self.0)
    }

    /// Makes the suffix part of the URL human readable by replacing
    /// underscores with spaces.
    ///
    /// # Examples
    ///
    /// ```
    /// use wglib::URL;
    ///
    /// let myUrl = URL::new("https://en.wikipedia.org/wiki/Help!_(film)").unwrap();
    ///
    /// assert_eq!(myUrl.get_name(), "Help! (film)");
    /// ```
    pub fn get_name(&self) -> String {
        str::replace(&self.0, "_", " ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_wikipedia_article_valid() -> Result<(), Box<dyn Error>> {
        URL::extract_body("https://en.wikipedia.org/wiki/Wikipedia")?;
        URL::extract_body("https://en.wikipedia.org/wiki/Help!_(film)")?;
        Ok(())
    }

    #[test]
    fn is_wikipedia_article_invalid() {
        if let Ok(_) = URL::extract_body("https://en.wikipedia.org/wiki/Help:Contents") {
            panic!("Test1 failed.");
        }
        if let Ok(_) = URL::extract_body("https://en.wikipedia.org/wiki/Wikipedia:Contact_us") {
            panic!("Test 2 failed.")
        }
    }
}
