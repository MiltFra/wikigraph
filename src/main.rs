use futures::executor::block_on;
use std::env;
use std::process;
use tokio::prelude::*;
use wglib::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = wglib::run(cfg).await {
        eprintln!("Problem runnig program: {}", e);
    }
    Ok(())
}
