use std::env;
use std::process;

use clap::Parser;
use mgrep::Config;


fn main() {
    let config = Config::parse();

    if let Err(error) = mgrep::run(config) {
        eprintln!("mgrep error: {error}");
        process::exit(1);
    }
}