mod errors;
mod types;
mod rules;
mod enums;
mod utils;
mod cli;

use std::env;
use cli::cli::*;



fn main() {
    let args: Vec<String> = env::args().collect();
    lint_files(args[1..].to_vec());
}