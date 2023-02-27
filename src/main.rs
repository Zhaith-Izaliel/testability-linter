mod errors;
mod types;
mod rules;
mod enums;
mod utils;

use rules::java::java_rules::{
    parse_file,
    check_void
};
use std::path::Path;
use utils::print::*;

fn main() {
    let path = Path::new("/home/zhaith/Development/Studies/research-ensimag/testability-linter/example/Example");
    let class_file = parse_file(path).unwrap();
    let result = check_void(class_file, path.to_str().unwrap());
    print_lint(result);
}