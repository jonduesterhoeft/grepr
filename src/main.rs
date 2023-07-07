use clap::Parser;
use mgrep::Args;
use std::process;

fn main() {
    let args = Args::parse();

    if let Err(e) = mgrep::run(args) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
