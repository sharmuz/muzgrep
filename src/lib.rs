use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("missing required arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for result in search(&config.query, &contents) {
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
}
