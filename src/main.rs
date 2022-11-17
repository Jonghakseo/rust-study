extern crate study_main;

use std::{env, process};

use study_main::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    if let Err(err) = study_main::run(config) {
        println!("Application Error: {}", err);

        process::exit(1);
    }
}

