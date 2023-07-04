use std::fs;
use std::error::Error;

use clap::Parser;


/// A struct that stores the configuration parameters.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub query: String,
    pub path: String,
    #[arg(short, long)]
    ignore_case: bool,
    #[arg(short = 'v', long)]
    invert_match: bool,

}

/// Executes the search and outputs results.
pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&args.path)?;
    let results = search(&args, &contents)?;
    write(&results, &mut std::io::stdout())?;

    Ok(())
}

/// Searchs the file path for the query string.
fn search<'a>(args: &Args, contents: &'a str) -> Result<Vec<&'a str>, Box<dyn Error>> {
    let mut results = Vec::new();

    if !args.ignore_case {
        search_case_sensitive(&args.query, &contents, &mut results);
    } else {
        search_case_insensitive(&args.query, &contents, &mut results);
    }
    
    Ok(results)
}

// Case sensitive search
fn search_case_sensitive<'a>(query: &str, contents: &'a str, results: &mut Vec<&'a str>) {
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
}

// Case INsensitive search
fn search_case_insensitive<'a>(query: &str, contents: &'a str, results: &mut Vec<&'a str>) {
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
}

/// Writes the search results to the command line.
fn write<'a>(result: & Vec<&'a str>, mut writer: impl std::io::Write) -> Result<(), Box<dyn Error>> {
    for line in result {
        writeln!(writer, "{}", line)?;
    }
    
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_case_sensitive() {
        let query = "test";
        let mut results = Vec::new();
        let contents = "This is:\nA test function";
        search_case_sensitive(&query, &contents, &mut results);
        assert_eq!(vec!["A test function"], results)
    }

    #[test]
    fn test_search_case_insensitive() {
        let query = "TEST";
        let mut results = Vec::new();
        let contents = "This is:\nA test function";
        search_case_insensitive(&query, &contents, &mut results);
        assert_eq!(vec!["A test function"], results)
    }
}