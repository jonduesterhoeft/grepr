use std::env;
use std::process;
use clap::Parser;
use mgrep::Args;


fn main() {
    let args = Args::parse();

    if let Err(e) = mgrep::run(args) {
        println!("Application error: {e}");
        process::exit(1);
    }
}


