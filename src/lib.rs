use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use clap::Parser;


/// A structure that stores the configuration parameters.
#[derive(Parser)]
pub struct Config {
    query: String,
    path: std::path::PathBuf
}

/// Executes the search and outputs results.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = read(&config.path)?;
    let results = search(&config.query, &contents)?;

    write(&results, &mut std::io::stdout())?;

    Ok(())
}

fn read(path: &std::path::PathBuf) -> BufReader {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    Ok(reader)
}

/// Searchs the file path for the query string.
fn search<'a>(query: &str, contents: &BufReader) -> Result<Vec<&'a str>, Box<dyn Error>> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line.unwrap());
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

        assert_eq!(vec!["A test function"], search(query, contents))
    }
}