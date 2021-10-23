use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let config = match args.len() {
            4 =>{ 
                let case_sensitive = 
                    if env::var("CASE_INSENSITIVE").is_err() {
                        !args[1].contains("-i") 
                    } else {
                        false
                    };
                let query = args[2].clone();
                let filename = args[3].clone(); 
                Config {query, filename, case_sensitive}
            },

            _ => {
                let query = args[1].clone();
                let filename = args[2].clone(); 
                let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
                Config {query, filename, case_sensitive}
            },         
        };

        Ok(config)
    }
}

// memo: Box<Error> -> Box<dyn Error>
// trait objects without an explicit `dyn` are deprecated
// this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2021 edition!
// note: for more information, see issue #80165 <https://github.com/rust-lang/rust/issues/80165>
pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod test {
    use super::*;

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
            search_case_insensitive(query, contents)
        );
    }
}