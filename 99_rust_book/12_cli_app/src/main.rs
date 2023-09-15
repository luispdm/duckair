use std::{env, process};

use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {}", err);
        process::exit(1)
    });

    println!("searching for {}", config.query);
    println!("in {}", config.filename);

    if let Err(e) = run(config) {
        println!("something went wrong reading the file: {}", e);
        process::exit(1);
    }
}
