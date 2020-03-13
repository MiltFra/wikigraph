use std::error::Error;
use std::fs;
use thiserror::Error;

mod scraper;

/// ConfigErr is an enum that contains possible error values that
/// could occur during the Configuration of this library in Config::new.
#[derive(Error, Debug)]
pub enum ConfigErr {
  #[error("Found too many arguments. (expected: 2, found: {0})")]
  TooManyArguments(usize),
  #[error("Found too few arguments. (expected: 2, found: {0})")]
  TooFewArguments(usize),
  #[error("Could not parse search depth. (found {0})")]
  IntParseError(String),
  #[error("Found no valid urls in the file.")]
  NoValidUrls,
}
/// Config is a struct used to bundle all the possible configurations
/// for the wikigraph library.
pub struct Config {
  /// Contains a list of URLs to valid Wikipedia articles.
  articles: Vec<String>,
  /// Contains the depth for the search in the Wikipedia graph.
  depth: u32,
}

impl Config {
  /// Given an array of (command-line) arguments this function creates a fitting
  /// configurations file.
  /// 
  /// Excatly two arguments are expected, otherwise an error is returned.
  /// - An integer containing the desired search depth.
  /// - A file name containing the starting URLs.
  pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
    let len = args.len();
    // Checking args for len==2
    if len < 3 {
      return Err(Box::new(ConfigErr::TooFewArguments(len)));
    }
    if len > 3 {
      return Err(Box::new(ConfigErr::TooManyArguments(len)));
    }
    // Parsing the depth n
    let r = args[0].parse();
    let n;
    match r {
      Err(_) => return Err(Box::new(ConfigErr::IntParseError(args[0].clone()))),
      Ok(v) => n = v,
    }
    // Parsing the URL file
    let r = Config::get_urls(&args[2]);
    match r {
      Err(e) => return Err(e),
      Ok(v) => {
        return Ok(Config {
          articles: v,
          depth: n,
        })
      }
    }
  }

  /// Filters all the valid Wikipedia articles from a given String.
  /// Articles have to be on separate lines and follow the criteria specified in the scraper module.
  fn get_urls(path: &String) -> Result<Vec<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let contents = contents.lines();
    let mut valid_urls = Vec::new();
    for line in contents {
      let line = line.trim_end_matches('\n');
      if let Err(e) = scraper::validate_wikipedia_article(line) {
        eprintln!("Encountered invalid url: {}, {}", line, e);
        continue;
      }
      valid_urls.push(String::from(line));
    }
    if valid_urls.len() == 0 {
      return Err(Box::new(ConfigErr::NoValidUrls));
    }
    return Ok(valid_urls);
  }
}

/// A struct representing a Wikipedia article with attributes like
/// the URL, the Head Line, related articles, etc.
pub struct Article {}

/// The main function of this library. Running this allows you to find a
/// graph around a certain set of Wikipedia articles and possibly the shortest
/// paths between them.
pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
  let resp = reqwest::blocking::get("https://en.wikipedia.org/wiki/Pragma_once")?.text()?;
  println!("{:#?}", resp);
  Ok(())
}
