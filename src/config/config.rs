use toml::Table;
use std::{ path::PathBuf, fs};
use crate::errors::generic::GenericErrorKind;
use crate::types::rule::{ RuleKind, Rule };

pub fn read_config(config_file: String) -> Result<Table, GenericErrorKind> {
    let path = PathBuf::from(config_file.as_str());

    let Ok(content) = fs::read_to_string(path) else {
        return Err(GenericErrorKind::InvalidPath);
    };

    match content.parse::<Table>() {
        Ok(table) => Ok(table),
        Err(_) => Err(GenericErrorKind::ParseError),
    }
}

pub fn select_rules(table: Table) -> Option<Vec<Rule>> {
    let mut vector: Vec<Rule> = Vec::new();

    RuleKind::iterator().for_each(|kind| {
        let rule = Rule::new(&table, kind.to_key());
        match rule {
            Some(rule) => vector.push(rule),
            None => (),
        }
    });

    if vector.is_empty() {
        return None;
    }

    Some(vector)
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
    fn test_read_config(#[case] file: &str, #[case] expected: bool) {
        let result = read_config(String::from(file));
        assert_eq!(result.is_ok(), expected);
    }

    #[rstest]
    #[case::all_rules("tests/inputs/config/valid/all_rules.toml", 3)]
    #[case::all_and_other_rules("tests/inputs/config/valid/unnecessary_rules.toml", 3)]
    #[case::some_rules("tests/inputs/config/valid/some_rules.toml", 1)]
    #[case::some_and_other_rules("tests/inputs/config/valid/some_unnecessary_rules.toml", 2)]
    fn test_select_rules(#[case] file: &str, #[case] expected: usize) {
        let table = read_config(String::from(file)).unwrap();
        let option_vec = select_rules(table);
        match option_vec {
            None => assert_eq!(0, expected),
            Some(vec) => assert_eq!(vec.len(), expected)
        }
    }
}

