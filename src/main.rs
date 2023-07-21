use clap::Parser;
use grepr::*;
use std::process;


fn main() {
    let args = CommandArgs::parse();

    if let Err(e) = args.run() {
        println!("Application error: {e}");
        process::exit(1);
    }
}