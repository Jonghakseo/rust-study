use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn run (config:Config) -> Result<(), Box<dyn Error>>{
    let file = File::open(&config.filename)?;
    let file_text = get_string_from_file(file)?;

    println!("{}", file_text);
    Ok(())
}

fn get_string_from_file(mut file: File) -> Result<String, Box<dyn Error>> {
    let mut file_text = String::new();
    file.read_to_string(&mut file_text)?;

    Ok(file_text)
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        match args.len() {
            3 => {
                let query = args[1].clone();
                let filename = args[2].clone();

                Ok(Config { query, filename })
            }
            _ => Err("입력 인자의 갯수가 일치하지 않습니다.")
        }
    }
}
