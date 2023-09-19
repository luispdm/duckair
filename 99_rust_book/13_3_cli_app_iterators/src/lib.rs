use std::{env, error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    // remember: Rust lets you return values from if statements
    let res = if config.case_sensitive {
        search_case_sensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for l in res {
        println!("{}", l);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // implementing new with &str to show the usage of &'static
    // (the code at 12_cli_app returns Result<Config, String>)
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // the first element is the path to the program - we don't need it

        let query = match args.next() {
            Some(q) => q,
            None => return Err("query string missing"),
        };

        let filename = match args.next() {
            Some(f) => f,
            None => return Err("filename missing"),
        };

        let case_sensitive = env::var("CASE_SENSITIVE").is_ok();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines().filter(|l| l.contains(query)).collect()
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|l| l.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "me";
        let content = "hey\nit's me\nnot mr. MEME";

        assert_eq!(vec!["it's me"], search_case_sensitive(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "ME";
        let content = "hey\nit's me\nnot mr. MEME";

        assert_eq!(vec!["it's me", "not mr. MEME"], search(query, content));
    }
}
