use std::env;
use std::fs;
use std::process;
use std::error::Error;

pub struct Config{
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    
    let results = if config.ignore_case{
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results{
        println!("{line}");
    }
    Ok(()) 
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

impl Config{
    // with : impl Iterator<Item = String>
    // means that build function can work with any input args that
    // implements Iterator<String> trait
    pub fn build(mut args: impl Iterator<Item = String>)-> Result<Config, &'static str>{
        // skip the first argument
        args.next(); 

        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Did not get a query string"),
        };

        let file_path = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn!t get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config{query, file_path, ignore_case})
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn one_case(){
        let query = "duct";
        let contents = "\
Rust: 
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}









