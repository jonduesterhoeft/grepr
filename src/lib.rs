
/// A structure that stores the configuration parameters.
struct Config {
    query: String,
    path: String
}

/// Parses the command line arguments into the query and file path.
impl Config {
    fn new(args: &[String]) -> Config {
        // args[0] is taken up by the program's name
        let query = &args[1].clone();
        let path = &args[2].clone();

        Config { query, path }
    }
}