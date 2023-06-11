mod errors;
mod types;
mod rules;
mod utils;
mod cli;
mod config;

use std::env;
use cli::cli::*;



fn main() {
    let args: Vec<String> = env::args().collect();

    let config_file = match args.get(1) {
        Some(file) => file.to_owned(),
        None => { eprintln!("You didn't put a config file"); return (); }
    };

    let rules = match create_rules_list(config_file) {
        Some(rules) => rules,
        None => { eprintln!("You didn't select rules in the config file"); return () }
    };
    rules.iter().for_each(|rule| {
        lint_files(args[2..].to_vec(), rule);
    });
}

