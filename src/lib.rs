use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // Static constructor
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        /*
            We’re using the is_err method on the Result to check whether it’s an error and therefore unset,
            which means it should do a case-sensitive search.
            If the CASE_INSENSITIVE environment variable is set to anything,
            is_err will return false and the program will perform a case-insensitive search.
            We don’t care about the value of the environment variable, just whether it’s set or unset,
            so we’re checking is_err rather than using unwrap, expect,
            or any of the other methods we’ve seen on Result.
        */
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? operator means that the error will be returned from func in case one happens
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    }   else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(()) // returning () means the function is void, we use it for its side-effects only
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&'a str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results: Vec<&'a str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_config() {
        let config = Config::new(&[
            String::from("path"),
            String::from("query"),
            String::from("filename"),
        ]);
        assert!(config.is_ok());

        let unwrapped_config_value = config.unwrap();
        assert_eq!(unwrapped_config_value.filename, "filename");
        assert_eq!(unwrapped_config_value.query, "query");
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents),
        );
    }
}
