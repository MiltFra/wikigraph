use std::env;
use std::process;
use wglib::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = wglib::run(cfg) {
        eprintln!("Problem runnig program: {}", e); 
    }
}
