
/// A structure that stores the configuration parameters.
struct Config {
    query: String,
    path: String
}


impl Config {
    /// Parses the command line arguments into the query and file path.
    fn build(args: &[String]) -> Result<Config, &'static str> {
        // args[0] is taken up by the program's name
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        
        let query = args[1].clone();
        let path = args[2].clone();

        Ok(Config { query, path })
    }
}