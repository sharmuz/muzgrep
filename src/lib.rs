use std::error::Error;
use std::env;
use std::fs;

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();  // Gives name of program

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query string specified"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("No file path specified")
        };
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
    contents
        .lines()
        .filter(|l| l.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query_lower = query.to_lowercase();
    
    contents
        .lines()
        .filter(|l| l.to_lowercase().contains(&query_lower))
        .collect()
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
