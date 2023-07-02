use std::env;
use std::fs;

use mini-grep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    
    
    println!("In {path}");

    let contents = fs::read_to_string(config.path)
        .expect("read {config.path}")

    println!("With text:\n{contents}");
}