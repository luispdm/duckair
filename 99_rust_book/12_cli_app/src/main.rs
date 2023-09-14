use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);
    println!("searching for {}", config.query);
    println!("in {}", config.filename);

    let content = fs::read_to_string(config.filename).expect("something went wrong reading the file");
    println!("with text:\n{}", content);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    // clone not efficient because it copies data, but easier than using lifetimes for now
    let query = args[1].clone();
    let filename = args[2].clone();
    Config { query, filename }
}
