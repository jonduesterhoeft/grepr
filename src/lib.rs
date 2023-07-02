use std::fs;
use std::error::Error;


/// A structure that stores the configuration parameters.
pub struct Config {
    query: String,
    path: String
}

impl Config {
    /// Parses the command line arguments into the query and file path.
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // args[0] is taken up by the program's name
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        
        let query = args[1].clone();
        let path = args[2].clone();

        Ok(Config { query, path })
    }
}


/// Executes the search and outputs results.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)?;

    let results = search(&config.query, &contents);

    write(results, &mut std::io::stdout());

    Ok(())
}

/// Searchs the file path for the query string.
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    
    results
}

/// Writes the search results to the command line.
fn write(result: & Vec<&'a str>, mut writer: impl std::io::Write) {
    for line in result {
        writeln!(writer, "{}", line);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_file() {
        let query = "sunbeam";
        let path = "test/pale_blue_dot.txt";
        let config = Config { query, path };

        let contents = fs::read_to_string(config.path)?;
        let results = search(&config.query, &contents);

        let mut output = Vec::new();
        write(results, &mut output);

        assert_eq!(output, "on a mote of dust suspended in a sunbeam.");
    }

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