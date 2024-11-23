use std::fs;
use std::error::Error;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &file_contents) {
        println!("Found line: \"{line}\"");
    }

    Ok(())
}
 
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments! 3 expected")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn search<'a>(query: &str, file_contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in file_contents.lines() {
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
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}