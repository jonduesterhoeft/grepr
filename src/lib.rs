use std::fs;
use std::error::Error;

use clap::Parser;


/// A struct that stores the configuration parameters.
#[derive(Parser)]
pub struct Args {
    pub query: String,
    pub path: String
}

/// Executes the search and outputs results.
pub fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(args.path)?;

    let results = search(&args.query, &contents)?;

    write(&results, &mut std::io::stdout())?;

    Ok(())
}

/// Searchs the file path for the query string.
fn search<'a>(query: &str, contents: &'a str) -> Result<Vec<&'a str>, Box<dyn Error>> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    
    Ok(results)
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
    fn test_search() {
        let query = "test";
        let contents = "\
This is:
A test function
";
        let results = search(query, contents);

        let results = match results {
            Ok(r) => r,
            Err(e) => panic!("{:?}", e),
        };

        assert_eq!(vec!["A test function"], results)
    }
}