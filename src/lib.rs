use std::fs;
use std::path::PathBuf;
use std::error::Error;
use clap::Parser;
use regex::bytes::Regex;
use colored::*;


/// A minimal implementation of grep in Rust.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    query: String,
    path: PathBuf,
    #[arg(short, long)]
    ignore_case: bool,
    #[arg(short = 'v', long)]
    invert_match: bool,
    #[arg(short, long)]
    word: bool,
    #[arg(short, long)]
    line: bool,
}

struct Search<'a> {
    contents: &'a str,
    results: Vec<(usize, &'a str)>,
}

pub trait Run {
    fn run(&self) -> Result<(), Box<dyn Error>>;
    fn read(&self) -> Result<String, Box<dyn Error>>;
}

impl Run for Args {
    /// Executes the search and outputs results.
    fn run(&self) -> Result<(), Box<dyn Error>> {
        let contents = self.read()?;
        let mut search = Search::new(&contents);
        search.find(&self)?;
        search.write(&self, &mut std::io::stdout())?;
        Ok(())
    }

    /// Reads data from the file
    fn read(&self) -> Result<String, Box<dyn Error>> {
        let contents = fs::read_to_string(&self.path)?;
        Ok(contents)
    }
}

impl<'a> Search<'a> {
    pub fn new(contents: &'a str) -> Search<'a> {
        Search { contents, results: Vec::new() }
    }
}

trait IsSearch {
    fn find(&mut self, args: &Args) -> Result<(), Box<dyn Error>>;
    fn write(&self, args: &Args, writer: &mut impl std::io::Write) -> Result<(), Box<dyn Error>>;
}

impl<'a> IsSearch for Search<'a> {
    /// Searchs the file path for the query string.
    fn find(&mut self, args: &Args) -> Result<(), Box<dyn Error>> {
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

    /// Writes the search results to the command line.
    fn write(&self, args: &Args, writer: &mut impl std::io::Write) -> Result<(), Box<dyn Error>> {
        writeln!(writer, "{}", args.path.display())?;
        for (number, line) in &self.results {
            let colored_line = line.replace(&args.query, &args.query.red().bold().to_string());
            writeln!(writer, "{number}: {}", colored_line)?;
        }
        Ok(())
    }

}

// helper methods

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

        let args = Args { 
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

        let args = Args { 
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

        let args = Args { 
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

        let args = Args { 
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

        let args = Args { 
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

        let args = Args { 
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

        let args = Args { 
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

        let args = Args { 
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

        let args = Args { 
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

        let args = Args { 
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

        let args = Args { 
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

        let args = Args { 
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

        let args = Args { 
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

        let args = Args { 
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

        let args = Args { 
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

        let args = Args { 
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

        let args = Args { 
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

        let args = Args { 
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