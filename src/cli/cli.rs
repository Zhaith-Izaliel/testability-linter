use classfile_parser::ClassFile;

use crate::rules::java::parse::parse_file;
use crate::config::config::*;
use crate::types::rule::*;

pub fn lint_files(files: Vec<String>, rule: &Rule) -> i32 {
    let class_files: Vec<(ClassFile, &String)> = files
        .iter()
        .filter_map(|file| match parse_file(&file) {
            Ok(class_file) => Some((class_file, file)),
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        })
        .collect();

    let mut code = 0;

    class_files.iter().for_each(|class_file| {
        let result = rule.run(&class_file.0, class_file.1);
        if result.result().is_err() {
            code = 1;
        }
        println!("{}", result);
    });
    return code;
}

pub fn create_rules_list(config_file: String) -> Option<Vec<Rule>> {
    let table = match read_config(config_file) {
        Ok(table) => table,
        Err(e) => { eprintln!("{}", e); return None; }
    };

    select_rules(table)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::empty("", false)]
    #[case::malformed("tests/inputs/config/invalid/malformed_file.toml", false)]
    #[case::all_rules("tests/inputs/config/valid/all_rules.toml", true)]
    #[case::all_and_other_rules("tests/inputs/config/valid/unnecessary_rules.toml", true)]
    #[case::some_rules("tests/inputs/config/valid/some_rules.toml", true)]
    #[case::some_and_other_rules("tests/inputs/config/valid/some_unnecessary_rules.toml", true)]
    fn test_create_rules_list(#[case] file: &str, #[case] expected: bool) {
        let option_vec = create_rules_list(String::from(file));
        assert_eq!(option_vec.is_some(), expected);
    }

    #[rstest]
    #[case::empty("", 0)]
    #[case::malformed("tests/inputs/config/invalid/malformed_file.toml", 0)]
    #[case::all_rules("tests/inputs/config/valid/all_rules.toml", 3)]
    #[case::all_and_other_rules("tests/inputs/config/valid/unnecessary_rules.toml", 3)]
    #[case::some_rules("tests/inputs/config/valid/some_rules.toml", 1)]
    #[case::some_and_other_rules("tests/inputs/config/valid/some_unnecessary_rules.toml", 2)]
    fn test_create_rules_list_number(#[case] file: &str, #[case] expected: usize) {
        let option_vec = create_rules_list(String::from(file));
        match option_vec {
            None => assert_eq!(0, expected),
            Some(vec) => assert_eq!(vec.len(), expected)
        }
    }
}

