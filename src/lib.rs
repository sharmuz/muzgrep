use std::error::Error;
use std::env;
use std::fs;

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("missing required arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for result in results {
        println!("{result}");
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
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        // SETUP
        let query = "pen";
        let contents = "\
            Haha!\
            \nCan't catch me for a penny cup of tea!\
            \nGoodbye.";
        let expected = vec!["Can't catch me for a penny cup of tea!"];

        // CALL
        let result = search(query, contents);
        
        // ASSERT
        assert_eq!(result, expected);
    }

    #[test]
    fn case_sensitive() {
        // SETUP
        let query = "pen";
        let contents = "\
            My pen!\
            \nPenny gave it to me.";
        let expected = vec!["My pen!"];

        // CALL
        let result = search(query, contents);
        
        // ASSERT
        assert_eq!(result, expected);
    }

    #[test]
    fn case_insensitive() {
        // SETUP
        let query = "pen";
        let contents = "\
            My pen!\
            \nPenny gave it to me.";
        let expected = vec!["My pen!", "Penny gave it to me."];

        // CALL
        let result = search_case_insensitive(query, contents);
        
        // ASSERT
        assert_eq!(result, expected);
    }
}
