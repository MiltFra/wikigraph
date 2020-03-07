use std::error::Error;
use std::fs;
use std::vec;
use thiserror::Error;

mod scraper;

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
pub struct Config {
  articles: Vec<String>,
  depth: u32,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
    let len = args.len();
    if len < 3 {
      return Err(Box::new(ConfigErr::TooFewArguments(len)));
    }
    if len > 3 {
      return Err(Box::new(ConfigErr::TooManyArguments(len)));
    }
    let r = args[0].parse();
    let n;
    match r {
      Err(e) => return Err(Box::new(ConfigErr::IntParseError(args[0].clone()))),
      Ok(v) => n = v,
    }
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

pub struct Article {}

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
  let resp = reqwest::blocking::get("https://en.wikipedia.org/wiki/Pragma_once")?.text()?;
  println!("{:#?}", resp);
  Ok(())
}
