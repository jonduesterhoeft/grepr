use std::env;
use std::fs;
use std::process;

use mini-grep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args)
        .unwrap_or_else(|err| {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        });
    
    println!("In {path}");

    run(config);
    
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.path)
        .expect("read {config.path}")

    println!("With text:\n{contents}");
}