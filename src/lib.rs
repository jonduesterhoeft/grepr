//! A minimal implementation of grep in rust.
//! 
//! # Overview #
//! **grepr** is a simple command line search tool. A search string and 
//! file path are input as arguments, along with several optionals 
//! to fine tune the search. The program iterates through each line in the
//! specified file and will return any lines matching the search criteria.
//! 
//! # Examples #
//! A simple search example.
#![doc = include_str!("../examples/simple.md")]
//!
//! Search for an exact word. In this case any non-alphanumeric characters
//! are ignored.
#![doc = include_str!("../examples/exact_word.md")]
//!
//! Inverting the search results. All lines without a match are returned.
#![doc = include_str!("../examples/invert.md")]
//!
use std::fs;
use std::path::PathBuf;
use std::error::Error;
use clap::Parser;
use regex::bytes::Regex;
use colored::*;


/// A parser for command line input.
/// 
/// Reads the `query` and `path` arguments for the search along with a 
/// number of options from the command line.
/// 
/// # Options #
#[doc = include_str!("../examples/help.md")]
///
#[derive(Parser)]
#[command(version, about = "A simple to use command line search tool, à la grep.", long_about = None)]
pub struct CommandArgs {
    /// Search query
    query: String,
    /// File path
    path: PathBuf,
    #[arg(short, long)]
    /// Ignores case whiles searching
    ignore_case: bool,
    #[arg(short = 'v', long)]
    /// Inverst search results
    invert_match: bool,
    #[arg(short, long)]
    /// Matches exact words only
    word: bool,
    #[arg(short, long)]
    /// Matches exact lines only
    line: bool,
}

/// Stores the results of the search and a reference to the contents.
/// 
/// `Search` is used in conjunction wih `CommandsArgs` which contains
/// the specific parameters used for the search.
/// 
pub struct Search<'a> {
    contents: &'a str,
    results: Vec<(usize, &'a str)>,
}

/// Defines methods expected to run on `CommandArgs`.
pub trait RunArgs {
    /// Executes the search process given the command line arguments.
    fn run(&self) -> Result<(), Box<dyn Error>>;

    /// Reads and stores the contents of a file.
    fn read(&self) -> Result<String, Box<dyn Error>>;
}

impl CommandArgs {
    /// Creates a new `CommandArgs`.
    /// 
    /// # Returns
    /// Returns a `CommandArgs` containing the specified arguments.
    /// 
    /// # Example
    /// ```
    /// # use crate::grepr::CommandArgs;
    /// # use std::path::PathBuf;
    /// let query = "this is a test.".to_string();
    /// let path = PathBuf::new();
    /// let contents = "this is a test.\nthis is another test!";
    /// let ignore_case = false;
    /// let invert_match = false;
    /// let word = false;
    /// let line = true;
    /// 
    /// let new_args = CommandArgs::new(
    ///     query, 
    ///     path, 
    ///     ignore_case, 
    ///     invert_match, 
    ///     word, 
    ///     line
    /// );
    /// ```
    /// 
    pub fn new(query: String, path: PathBuf, ignore_case: bool, invert_match: bool, word: bool, line: bool) -> CommandArgs {
        CommandArgs {
            query, 
            path,
            ignore_case,
            invert_match,
            word,
            line 
        }
    }
}

impl RunArgs for CommandArgs {
    /// Executes the search process given the command line arguments.
    /// 
    /// Reads contents of the specified file and generates a new 
    /// `Search` struct to store the results. Once completed, the results
    /// are written to the terminal.
    /// 
    /// # Returns
    /// Returns () if successful.
    /// 
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let contents = self.read()?;
        let mut search = Search::new(&contents);
        search.find(&self)?;
        search.write(&self, &mut std::io::stdout())?;
        Ok(())
    }

    /// Reads and stores the contents of a file.
    /// 
    /// # Returns
    /// Returns the contents of a file as a `String`.
    /// 
    fn read(&self) -> Result<String, Box<dyn Error>> {
        let contents = fs::read_to_string(&self.path)?;
        Ok(contents)
    }
}


/// Defines methods expected to run on `Search`.
pub trait IsSearch {
    /// Searchs for the query in the file contents.
    fn find(&mut self, args: &CommandArgs) -> Result<(), Box<dyn Error>>;
}

impl<'a> Search<'a> {
    /// Creates a new `Search`.
    /// 
    /// # Returns
    /// Returns a `Search` containing a reference to `contents` 
    /// and an empty `results` vector.
    /// 
    /// # Example
    /// ```
    /// # use crate::grepr::Search;
    /// let some_text = "This is a test.\n With two lines.".to_string();
    /// 
    /// let new_search = Search::new(&some_text);
    /// ```
    /// 
    pub fn new(contents: &'a str) -> Search<'a> {
        Search { contents, results: Vec::new() }
    }

    /// Writes the search results to the command line.
    fn write(&self, args: &CommandArgs, writer: &mut impl std::io::Write) -> Result<(), Box<dyn Error>> {
        writeln!(writer, "{}", args.path.display())?;
        for (number, line) in &self.results {
            let colored_line = line.replace(&args.query, &args.query.red().bold().to_string());
            writeln!(writer, "{number}: {}", colored_line)?;
        }
        Ok(())
    }

    /// Returns the raw results vector from `Search`.
    pub fn get_results(&self) -> &Vec<(usize, &'a str)> {
        &self.results
    }
}

impl<'a> IsSearch for Search<'a> {
    /// Searchs the file path for the query string.
    fn find(&mut self, args: &CommandArgs) -> Result<(), Box<dyn Error>> {
        let query = prep_string(&args.query.to_string(), args.ignore_case);
        let word_regex = Regex::new(r"\W+").unwrap();
        for (number, line) in self.contents.lines().enumerate() {
            let search_line = prep_string(line, args.ignore_case);

            let line_match = args.line && search_line == query;
            let word_match = !args.line && args.word && word_regex.split(&search_line).any(|word| word == query);
            let partial_match = !args.line && !args.word && search_line.windows(query.len()).any(|window| window == query);

            let match_found: bool = line_match || word_match || partial_match;
            
            if match_found && !args.invert_match || !match_found && args.invert_match {
                self.results.push((number, line));
            }
        }

        Ok(())
    }
}


// helper methods

// Prepares a string for saerch.
// The string is converted to lowercase if lower == true.
// Either way, the string is converted to bytes ahead of the search.
fn prep_string(str: &str, lower: bool) -> Vec<u8> {
    if lower {
        str.to_lowercase().into_bytes()
    } else {
        str.to_string().into_bytes()
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_line_case_noinvert_good() {
        let query = "this is a test.".to_string();
        let path = PathBuf::new();
        let contents = "this is a test.\nthis is another test!";
        let ignore_case = false;
        let invert_match = false;
        let word = false;
        let line = true;

        let args = CommandArgs { 
            query, 
            path,
            ignore_case,
            invert_match,
            word,
            line 
        };

        let mut search = Search::new(&contents);
        let _ = search.find(&args);

        assert_eq!(search.results[0].1, "this is a test.")
    }



    #[test]
    fn test_search_line_case_noinvert_bad() {
        let query = "this is a test".to_string();
        let path = PathBuf::new();
        let contents = "this is a test.\nthis is another test!";
        let ignore_case = false;
        let invert_match = false;
        let word = false;
        let line = true;

        let args = CommandArgs { 
            query, 
            path,
            ignore_case,
            invert_match,
            word,
            line 
        };

        let mut search = Search::new(&contents);
        let _ = search.find(&args);

        assert_eq!(search.results.len(), 0)
    }

    #[test]
    fn test_search_line_nocase_noinvert_good() {
        let query = "THIS is a test.".to_string();
        let path = PathBuf::new();
        let contents = "this is a test.\nthis is another test!";
        let ignore_case = true;
        let invert_match = false;
        let word = false;
        let line = true;

        let args = CommandArgs { 
            query, 
            path,
            ignore_case,
            invert_match,
            word,
            line 
        };

        let mut search = Search::new(&contents);
        let _ = search.find(&args);

        assert_eq!(search.results[0].1, "this is a test.")
    }



    #[test]
    fn test_search_line_nocase_noinvert_bad() {
        let query = "THIS is a test".to_string();
        let path = PathBuf::new();
        let contents = "this is a test.\nthis is another test!";
        let ignore_case = true;
        let invert_match = false;
        let word = false;
        let line = true;

        let args = CommandArgs { 
            query, 
            path,
            ignore_case,
            invert_match,
            word,
            line 
        };

        let mut search = Search::new(&contents);
        let _ = search.find(&args);

        assert_eq!(search.results.len(), 0)
    }

    #[test]
    fn test_search_line_nocase_invert_good() {
        let query = "THIS is a test.".to_string();
        let path = PathBuf::new();
        let contents = "this is a test.\nthis is another test!";
        let ignore_case = true;
        let invert_match = true;
        let word = false;
        let line = true;

        let args = CommandArgs { 
            query, 
            path,
            ignore_case,
            invert_match,
            word,
            line 
        };

        let mut search = Search::new(&contents);
        let _ = search.find(&args);

        assert_eq!(search.results[0].1, "this is another test!")
    }



    #[test]
    fn test_search_line_nocase_invert_bad() {
        let query = "THIS is a test".to_string();
        let path = PathBuf::new();
        let contents = "this is a test.\nthis is another test!";
        let ignore_case = true;
        let invert_match = true;
        let word = false;
        let line = true;

        let args = CommandArgs { 
            query, 
            path,
            ignore_case,
            invert_match,
            word,
            line 
        };

        let mut search = Search::new(&contents);
        let _ = search.find(&args);

        assert_eq!(search.results.len(), 2)
    }

    #[test]
    fn test_search_word_case_noinvert_good() {
        let query = "another".to_string();
        let path = PathBuf::new();
        let contents = "this is a test.\nthis is another test!";
        let ignore_case = false;
        let invert_match = false;
        let word = true;
        let line = false;

        let args = CommandArgs { 
            query, 
            path,
            ignore_case,
            invert_match,
            word,
            line 
        };

        let mut search = Search::new(&contents);
        let _ = search.find(&args);

        assert_eq!(search.results[0].1, "this is another test!")
    }



    #[test]
    fn test_search_word_case_noinvert_bad() {
        let query = "nothing".to_string();
        let path = PathBuf::new();
        let contents = "this is a test.\nthis is another test!";
        let ignore_case = false;
        let invert_match = false;
        let word = true;
        let line = false;

        let args = CommandArgs { 
            query, 
            path,
            ignore_case,
            invert_match,
            word,
            line 
        };

        let mut search = Search::new(&contents);
        let _ = search.find(&args);

        assert_eq!(search.results.len(), 0)
    }

    #[test]
    fn test_search_word_nocase_noinvert_good() {
        let query = "ANOTHER".to_string();
        let path = PathBuf::new();
        let contents = "this is a test.\nthis is another test!";
        let ignore_case = true;
        let invert_match = false;
        let word = true;
        let line = false;

        let args = CommandArgs { 
            query, 
            path,
            ignore_case,
            invert_match,
            word,
            line 
        };

        let mut search = Search::new(&contents);
        let _ = search.find(&args);

        assert_eq!(search.results[0].1, "this is another test!")
    }



    #[test]
    fn test_search_word_nocase_noinvert_bad() {
        let query = "NOTHING".to_string();
        let path = PathBuf::new();
        let contents = "this is a test.\nthis is another test!";
        let ignore_case = true;
        let invert_match = false;
        let word = true;
        let line = false;

        let args = CommandArgs { 
            query, 
            path,
            ignore_case,
            invert_match,
            word,
            line 
        };

        let mut search = Search::new(&contents);
        let _ = search.find(&args);

        assert_eq!(search.results.len(), 0)
    }

    #[test]
    fn test_search_word_nocase_invert_good() {
        let query = "another".to_string();
        let path = PathBuf::new();
        let contents = "this is a test.\nthis is another test!";
        let ignore_case = true;
        let invert_match = true;
        let word = true;
        let line = false;

        let args = CommandArgs { 
            query, 
            path,
            ignore_case,
            invert_match,
            word,
            line 
        };

        let mut search = Search::new(&contents);
        let _ = search.find(&args);

        assert_eq!(search.results[0].1, "this is a test.")
    }



    #[test]
    fn test_search_word_nocase_invert_bad() {
        let query = "nothing".to_string();
        let path = PathBuf::new();
        let contents = "this is a test.\nthis is another test!";
        let ignore_case = true;
        let invert_match = true;
        let word = true;
        let line = false;

        let args = CommandArgs { 
            query, 
            path,
            ignore_case,
            invert_match,
            word,
            line 
        };

        let mut search = Search::new(&contents);
        let _ = search.find(&args);

        assert_eq!(search.results.len(), 2)
    }



    #[test]
    fn test_search_partial_case_noinvert_good() {
        let query = "ano".to_string();
        let path = PathBuf::new();
        let contents = "this is a test.\nthis is another test!";
        let ignore_case = false;
        let invert_match = false;
        let word = false;
        let line = false;

        let args = CommandArgs { 
            query, 
            path,
            ignore_case,
            invert_match,
            word,
            line 
        };

        let mut search = Search::new(&contents);
        let _ = search.find(&args);

        assert_eq!(search.results[0].1, "this is another test!")
    }



    #[test]
    fn test_search_partial_case_noinvert_bad() {
        let query = "nothing".to_string();
        let path = PathBuf::new();
        let contents = "this is a test.\nthis is another test!";
        let ignore_case = false;
        let invert_match = false;
        let word = false;
        let line = false;

        let args = CommandArgs { 
            query, 
            path,
            ignore_case,
            invert_match,
            word,
            line 
        };

        let mut search = Search::new(&contents);
        let _ = search.find(&args);

        assert_eq!(search.results.len(), 0)
    }

    #[test]
    fn test_search_partial_nocase_noinvert_good() {
        let query = "ANO".to_string();
        let path = PathBuf::new();
        let contents = "this is a test.\nthis is another test!";
        let ignore_case = true;
        let invert_match = false;
        let word = false;
        let line = false;

        let args = CommandArgs { 
            query, 
            path,
            ignore_case,
            invert_match,
            word,
            line 
        };

        let mut search = Search::new(&contents);
        let _ = search.find(&args);

        assert_eq!(search.results[0].1, "this is another test!")
    }



    #[test]
    fn test_search_partial_nocase_noinvert_bad() {
        let query = "NOTHING".to_string();
        let path = PathBuf::new();
        let contents = "this is a test.\nthis is another test!";
        let ignore_case = true;
        let invert_match = false;
        let word = false;
        let line = false;

        let args = CommandArgs { 
            query, 
            path,
            ignore_case,
            invert_match,
            word,
            line 
        };

        let mut search = Search::new(&contents);
        let _ = search.find(&args);

        assert_eq!(search.results.len(), 0)
    }

    #[test]
    fn test_search_partial_nocase_invert_good() {
        let query = "ano".to_string();
        let path = PathBuf::new();
        let contents = "this is a test.\nthis is another test!";
        let ignore_case = true;
        let invert_match = true;
        let word = false;
        let line = false;

        let args = CommandArgs { 
            query, 
            path,
            ignore_case,
            invert_match,
            word,
            line 
        };

        let mut search = Search::new(&contents);
        let _ = search.find(&args);

        assert_eq!(search.results[0].1, "this is a test.")
    }



    #[test]
    fn test_search_partial_nocase_invert_bad() {
        let query = "nothing".to_string();
        let path = PathBuf::new();
        let contents = "this is a test.\nthis is another test!";
        let ignore_case = true;
        let invert_match = true;
        let word = false;
        let line = false;

        let args = CommandArgs { 
            query, 
            path,
            ignore_case,
            invert_match,
            word,
            line 
        };

        let mut search = Search::new(&contents);
        let _ = search.find(&args);

        assert_eq!(search.results.len(), 2)
    }

}