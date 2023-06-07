use classfile_parser::types::ClassFile;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    errors::{
        generic::*, fail::Fail
    },
    enums::rules_enum::*,
    types::rule::*
};

use super::utils::*;

pub fn no_binary_in_names(class_file: ClassFile, file: &str) -> RuleResult {
    let const_pool = &class_file.const_pool;

    let errors: Vec<Fail> = class_file.methods
        .iter()
        .filter_map(|method| {
            let name = match extract_utf8_constant(const_pool, method.name_index) {
                Ok(name) => name,
                Err(e) => return Some(
                    Fail::new(String::from("N/A"), e.message().clone(), e.kind())
                )
            };

            lazy_static! {
                static ref NO_BINARY_IN_NAMES_REGEX: Regex = Regex::new(
                    r"^(_?|.*_)(and|or|AND|OR)([A-Z]|_).+|.+[a-z](And|Or)[A-Z].*$"
                ).unwrap();
            }

            match NO_BINARY_IN_NAMES_REGEX.is_match(name.utf8_string.as_str()) {
                true => Some(Fail::new(
                    name.utf8_string.to_owned(),
                    String::from("This method's name contains and/or"),
                    GenericErrorKind::RuleCheckFailed
                )),
                false => None,
            }
        })
    .collect();

    RuleResult::new(
        String::from(file),
        Rules::NoBinaryInNames,
        match errors.is_empty() {
            true => Ok(()),
            false => Err(errors)
        }
    )
}

/* -------------------------------------------------------------------------- */
/*                                  Test Suit                                 */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::parse::parse_file;
    use std::{
        fs,
        path::Path,
    };

    struct LinterInputs(Vec<(String, ClassFile)>);

    impl LinterInputs {
        pub fn new(input: &str, rule: Rules, valid: bool) -> Self {
            let valid_string = if valid {"valid"} else {"invalid"};
            let dir = format!("{}/{}/{}", input, rule.to_dir_string(), valid_string);
            Self(Self::get_input_files(&dir))
        }

        fn get_input_files(dir: &String) -> Vec<(String, ClassFile)> {
            let path = Path::new(dir);
            fs::read_dir(path)
            .unwrap()
            .map(
                |x| {
                    let file = String::from(x.unwrap().path().to_str().unwrap());
                    let class_file = parse_file(&file).unwrap();
                    (file, class_file)
                }
            ).collect()
        }
    }

    const INPUTS: &str = "tests/inputs/java";

    #[test]
    fn parse_ok() {
        let inputs_valid = LinterInputs::new(INPUTS, Rules::NoBinaryInNames, true);
        let inputs_invalid = LinterInputs::new(INPUTS, Rules::NoBinaryInNames, false);

        assert!(!inputs_valid.0.is_empty());
        assert!(!inputs_invalid.0.is_empty());
    }

    #[test]
    #[should_panic]
    fn parse_fail() {
        let faulty_file = String::from("");
        parse_file(&faulty_file).unwrap();
    }

    #[test]
    fn no_binary_in_names_ok() {
        let inputs = LinterInputs::new(INPUTS, Rules::NoBinaryInNames, true);
        for (file, class_file) in inputs.0 {
            assert!(no_binary_in_names(class_file, file.as_str()).result().is_ok());
        }
    }

    #[test]
    fn no_binary_in_names_fail() {
        let inputs = LinterInputs::new(INPUTS, Rules::NoBinaryInNames, false);
        for (file, class_file) in inputs.0 {
            assert!(!no_binary_in_names(class_file, file.as_str()).result().is_ok());
        }
    }
}

