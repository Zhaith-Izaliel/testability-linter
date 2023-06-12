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
        None => { eprintln!("You didn't put a config file"); std::process::exit(1); }
    };

    let rules = match create_rules_list(config_file) {
        Some(rules) => rules,
        None => { eprintln!("You didn't select rules in the config file"); std::process::exit(1); }
    };
    let mut code = 0;
    rules.iter().for_each(|rule| {
        let rule_code = lint_files(args[2..].to_vec(), rule);
        if code != 1 { code = rule_code }
    });

    std::process::exit(code);
}

