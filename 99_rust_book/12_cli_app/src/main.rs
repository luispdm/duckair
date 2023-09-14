use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);
    println!("searching for {}", query);
    println!("in {}", filename);

    let content = fs::read_to_string(filename).expect("something went wrong reading the file");
    println!("with text:\n{}", content);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}
