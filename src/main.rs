use std::{env, error::Error, fs, process};

use minigrep::{search, search_case_insensitive};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

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

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() > 1 && (args[1] == "-h" || args[1] == "--help") {
            Config::return_help_text();
        }
        if args.len() < 3 {
            return Err("not enought arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        let mut ignore_case = false;

        for arg in args.iter().skip(3) {
            match arg.as_str() {
                "-i" | "--ignore-case" => {
                    ignore_case = true;
                }
                "-h" | "--help" => {
                    Config::return_help_text();
                }
                _ => {
                    eprintln!("Try 'grep --help' for more information");
                    process::exit(0);
                }
            }
        }

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

impl Config {
    fn return_help_text() {
        println!("Usage: minigrep <QUERY> <FILE_PATH> [OPTIONS]");
        println!("Options: -i/--ignore-case, -h/--help");
        process::exit(0);
    }
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}
