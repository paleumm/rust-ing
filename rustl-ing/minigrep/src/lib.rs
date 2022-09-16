use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // ignored : path to program

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get the file name"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_err();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result // vector of string
}

pub fn search_case_insentitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let result = if config.ignore_case {
        search_case_insentitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    // iterate through each element in vector
    for line in result {
        println!("{line}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
            search_case_insentitive(query, contents)
        );
    }
}
