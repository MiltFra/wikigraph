use std::env;
use std::process;
use wglib::config::Config;

fn main() {
    let cfg = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = wglib::run(cfg) {
        eprintln!("Problem runnig program: {}", e); 
    }
}
