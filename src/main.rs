use std::env;
use std::process;

use mgrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args)
        .unwrap_or_else(|err| {
            eprintln!("problem parsing arguments: {err}");
            process::exit(1);
        });

    if let Err(error) = mgrep::run(config) {
        eprintln!("mgrep error: {error}");
        process::exit(1);
    }
}