use std::env;
use std::process;

use image_proc::run;

pub struct Config {
    file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        match args.len() {
            n if n < 2 => Err("not enough arguments"),
            _ => Ok(Config {
                file_path: args[1].clone(),
            }),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });

    if let Err(err) = run(&config.file_path) {
        println!("Application error: {}", err)
    }
}
