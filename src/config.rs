use super::url::URL;
use std::error::Error;
use std::fs;
use thiserror::Error;
/// Contains the prefix that is used to identify Wikipedia articles.
///
/// Any wikipedia article must therefore be of the form
/// "<WIKI_ARTICLE_PREFIX><ARTICLE_NAME>".
pub const WIKI_ARTICLE_PREFIX: &str = "/wiki/";

pub const WIKI_DOMAIN: &str = "https://en.wikipedia.org";

/// Contains prefixes of webpages that are not considered Wikipedia articles.
///
/// Any url of the form "<WIKI_ARTICLE_PREFIX><BLACKLIST_ELEMENT><REST>",
/// where BLACKLIST_ELEMENT is one of the elements in this array and
/// REST is the possibly empty rest of the string, is therefore invalid.
pub const WIKI_ARTICLE_PREFIX_BLACKLIST: [&str; 8] = [
    "Category:",
    "Talk:",
    "Main_Page",
    "Help:",
    "Wikipedia:",
    "Special:",
    "File:",
    "Portal:"
];

pub const WIKI_ARTICLE_SUFFIX_BLACKLIST: [&str; 1] = ["_(disambiguation)"];

pub const REFERENCE_PREFIX: &str = "<a href=\"";
/// ConfigErr is an enum that contains possible error values that
/// could occur during the Configuration of this library in Config::new.
#[derive(Error, Debug)]
pub enum ConfigErr {
    /// This error is returned when there are less arguments in the
    /// given iterator, than expected.
    #[error("Found too few arguments. (expected: 2)")]
    TooFewArguments,
    /// This error is returned when the given argument corresponding
    /// to the search depth could not be parsed into the appropriate number type.
    #[error("Could not parse search depth. (found {0})")]
    IntParseError(String),
    /// This error is returned when the given file does not contain *any* valid urls.
    ///
    /// If there are some invalid URLs, those will just be discarded, but at least one
    /// starting point is required.
    #[error("Found no valid urls in the file.")]
    NoValidUrls,
}
/// Config is a struct used to encapsulate all the possible configurations
/// for the wikigraph library.
pub struct Config {
    /// Contains a list of URLs to valid Wikipedia articles.
    pub urls: Vec<URL>,
    /// Contains the depth for the search in the Wikipedia graph.
    pub depth: u32,
}

impl Config {
    /// Given an iterator over the command line arguments, this will return
    /// an appropriate config struct.
    ///
    /// Excatly two arguments are expected, otherwise an error is returned.
    /// - An integer containing the desired search depth.
    /// - A file name containing the starting URLs.
    pub fn new(mut args: std::env::Args) -> Result<Self, Box<dyn Error>> {
        eprintln!("Creating config");
        // Dropping the name of the executable.
        args.next();
        // Parsing the depth.
        let n = match args.next() {
            Some(arg) => match arg.parse() {
                Ok(v) => v,
                Err(_) => return Err(Box::new(ConfigErr::IntParseError(arg))),
            },
            None => return Err(Box::new(ConfigErr::TooFewArguments)),
        };
        // Parsing the URL file
        let urls = match args.next() {
            Some(arg) => Config::get_urls(&arg),
            None => return Err(Box::new(ConfigErr::TooFewArguments)),
        };
        match urls {
            Err(e) => Err(e),
            Ok(v) => Ok(Config { urls: v, depth: n }),
        }
    }

    pub fn iter_urls(&self) -> std::slice::Iter<URL> {
        self.urls.iter()
    }

    /// Filters all the valid Wikipedia articles from a given String.
    /// Articles have to be on separate lines and follow the criteria specified in the scraper module.
    fn get_urls(path: &String) -> Result<Vec<URL>, Box<dyn Error>> {
        eprintln!("Parsing URLs");
        let contents = fs::read_to_string(path)?;
        let valid_urls = URL::new_list(&contents);
        if valid_urls.len() == 0 {
            return Err(Box::new(ConfigErr::NoValidUrls));
        }
        return Ok(valid_urls);
    }
}

// TODO: Write tests
