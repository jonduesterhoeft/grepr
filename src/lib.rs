use std::fs;
use std::error::Error;
use clap::Parser;


/// A minimal implementation of grep in Rust.

// The CLI argument parser
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    query: String,
    path: String,
    #[arg(short, long)]
    ignore_case: bool,
    #[arg(short = 'v', long)]
    invert_match: bool,
    #[arg(short, long)]
    word: bool,
    #[arg(short, long)]
    line: bool,
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
    // Formats the string, uses lowercase if the ignore_case flag is set
    let query = lower_case(&args.query.to_string(), args.ignore_case);
    // Search loop
    for line in contents.lines() {
        let fmt_line = lower_case(&line.to_string(), args.ignore_case);

        if args.line && match_component(&query, &fmt_line, args.invert_match) {
            results.push(line);
        } else if contains_component(&query, &fmt_line, args.invert_match) {
            if args.word {
                // for word in fmt_line.split(&['-', ' ', ':', '@', '.', '"', '\'', '_'][..]) {
                //     if contains_component(&query, &word, args.invert_match) {
                //         results.push(line);
                //         break;
                //     }
                // }   
            } else if !args.word {
                results.push(line);
            }
        }

        //TODO : Add line numbers
    }
    
    Ok(results)
}

fn lower_case(str: &String, lower: bool) -> String {
    if lower {
        str.to_lowercase()
    } else {
        str.to_string()
    }
}

fn match_component<'a>(target: &str, component: &'a str, invert: bool) -> bool {
    if target == component {
        return true && !invert;
    } else {
        return false || invert;
    }
}

fn contains_component<'a>(target: &str, component: &'a str, invert: bool) -> bool {
    if component.contains(target) {
        return true && !invert;
    } else {
        return false || invert;
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
    fn test_match_component_good_noinvert() {
        let target = "test";
        let component = "test";
        let invert = false;
        let result = match_component(&target, &component, invert);
        assert_eq!(result, true)
    }

    #[test]
    fn test_match_component_bad_noinvert() {
        let target = "test";
        let component = "not test";
        let invert = false;
        let result = match_component(&target, &component, invert);
        assert_eq!(result, false)
    }

    #[test]
    fn test_match_component_good_invert() {
        let target = "test";
        let component = "test";
        let invert = true;
        let result = match_component(&target, &component, invert);
        assert_eq!(result, false)
    }

    #[test]
    fn test_match_component_bad_invert() {
        let target = "test";
        let component = "not test";
        let invert = true;
        let result = match_component(&target, &component, invert);
        assert_eq!(result, true)
    }


    // #[test]
    // fn test_search_case_sensitive() {
    //     let query = "test";
    //     let invert_match = false;
    //     let mut results = Vec::new();
    //     let contents = "This is:\nA test function";
    //     search_case_sensitive(&query, &contents, &mut results, &invert_match);
    //     assert_eq!(vec!["A test function"], results)
    // }

    // #[test]
    // fn test_search_case_insensitive() {
    //     let query = "TEST";
    //     let invert_match = false;
    //     let mut results = Vec::new();
    //     let contents = "This is:\nA test function";
    //     search_case_insensitive(&query, &contents, &mut results, &invert_match);
    //     assert_eq!(vec!["A test function"], results)
    // }

    // #[test]
    // fn test_search_case_sensitive_invert() {
    //     let query = "test";
    //     let invert_match = true;
    //     let mut results = Vec::new();
    //     let contents = "This is:\nA test function";
    //     search_case_sensitive(&query, &contents, &mut results, &invert_match);
    //     assert_eq!(vec!["This is:"], results)
    // }

    // #[test]
    // fn test_search_case_insensitive_invert() {
    //     let query = "TEST";
    //     let invert_match = true;
    //     let mut results = Vec::new();
    //     let contents = "This is:\nA test function";
    //     search_case_insensitive(&query, &contents, &mut results, &invert_match);
    //     assert_eq!(vec!["This is:"], results)
    // }
}