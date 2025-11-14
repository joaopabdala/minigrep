use clap::Parser;
use std::{error::Error, fs, process};

use minigrep::{search, search_case_insensitive};

fn main() {
    let config = Config::parse();

    println!("Configuração de Execução: {:?}", config);
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}")
    }

    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    pub query: String,

    pub file_path: String,

    #[arg(short, long, default_value_t = false)]
    pub ignore_case: bool,
}
