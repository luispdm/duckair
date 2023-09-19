use std::{env, process};

use minigrep_iterators::{run, Config};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {}", err);
        process::exit(1)
    });

    println!("searching for {}", config.query);
    println!("in {}", config.filename);

    if let Err(e) = run(config) {
        eprintln!("something went wrong reading the file: {}", e);
        process::exit(1);
    }
}
