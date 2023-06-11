use classfile_parser::types::ClassFile;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    errors::{fail::Fail, generic::*},
    types::rule::*,
};

use super::utils::*;

pub fn no_binary_in_names(class_file: ClassFile, file: &str) -> RuleResult {
    let const_pool = &class_file.const_pool;

    let errors: Vec<Fail> = class_file
        .methods
        .iter()
        .filter_map(|method| {
            let name = match extract_method_name(const_pool, method.name_index) {
                Ok(name) => name,
                Err(e) => return Some(e),
            };

            lazy_static! {
                static ref NO_BINARY_IN_NAMES_REGEX: Regex =
                    Regex::new(r"^(_?|.*_)(and|or|AND|OR)([A-Z]|_).+|.+[a-z](And|Or)[A-Z].*$")
                        .unwrap();
            }

            match NO_BINARY_IN_NAMES_REGEX.is_match(name.as_str()) {
                true => Some(Fail::new(
                    name.to_owned(),
                    String::from("This method's name contains and/or"),
                    GenericErrorKind::RuleCheckFailed,
                )),
                false => None,
            }
        })
        .collect();

    RuleResult::new(
        String::from(file),
        RuleKind::NoBinaryInNames,
        match errors.is_empty() {
            true => Ok(()),
            false => Err(errors),
        },
    )
}

pub fn check_no_void(class_file: ClassFile, file: &str) -> RuleResult {
    let const_pool = &class_file.const_pool;

    let errors: Vec<Fail> = class_file
        .methods
        .iter()
        .filter_map(|method| {
            let name = match extract_method_name(const_pool, method.name_index) {
                Ok(name) => name,
                Err(e) => return Some(e),
            };

            let descriptor =
                match extract_method_descriptor(const_pool, method.descriptor_index, &name) {
                    Ok(descriptor) => descriptor,
                    Err(e) => return Some(e),
                };

            lazy_static! {
                static ref MATCH_CONSTRUCTORS_AND_MAIN: Regex =
                    Regex::new(r".*<init>.*|.*<clinit>.*|main").unwrap();
            }

            if MATCH_CONSTRUCTORS_AND_MAIN.is_match(name) {
                return None;
            }

            if &descriptor[descriptor.len() - 1..] == "V" {
                return Some(Fail::new(
                    name.to_owned(),
                    String::from("This method has return type of void"),
                    GenericErrorKind::RuleCheckFailed,
                ));
            }

            None
        })
        .collect();
    RuleResult::new(
        String::from(file),
        RuleKind::CheckNoVoid,
        match errors.is_empty() {
            true => Ok(()),
            false => Err(errors)
        }
    )
}

pub fn too_many_arguments(class_file: ClassFile, file: &str, max_arguments: u8) -> RuleResult {
    let const_pool = &class_file.const_pool;

    let errors: Vec<Fail> = class_file
        .methods
        .iter()
        .filter_map(|method| {
            let name = match extract_method_name(const_pool, method.name_index) {
                Ok(name) => name,
                Err(e) => return Some(e),
            };

            let descriptor =
                match extract_method_descriptor(const_pool, method.descriptor_index, &name) {
                    Ok(descriptor) => descriptor,
                    Err(e) => return Some(e),
                };
            if count_parameters(descriptor) > max_arguments {
                return Some(Fail::new(
                    name.to_owned(),
                    String::from(format!(
                        "This method has too many arguments (max: {})",
                        max_arguments
                    )),
                    GenericErrorKind::RuleCheckFailed,
                ));
            }
            None
        })
        .collect();

    RuleResult::new(
        String::from(file),
        RuleKind::TooManyArguments,
        match errors.is_empty() {
            true => Ok(()),
            false => Err(errors),
        },
    )
}

fn count_parameters(descriptor: &String) -> u8 {
    lazy_static! {
        static ref MATCH_PARAMETERS: Regex = Regex::new(r"L[^;]*|\(|\).*|").unwrap();
    }
    MATCH_PARAMETERS
        .split(descriptor)
        .filter(|s| !s.is_empty())
        .count() as u8
}

/* -------------------------------------------------------------------------- */
/*                                  Test Suit                                 */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::super::parse::parse_file;
    use super::*;
    use rstest::rstest;
    use std::{fs, path::Path};

    struct LinterInputs(Vec<(String, ClassFile)>);

    impl LinterInputs {
        pub fn new(input: &str, rule: RuleKind, valid: bool) -> Self {
            let valid_string = if valid { "valid" } else { "invalid" };
            let dir = format!("{}/{}/{}", input, rule.to_key(), valid_string);
            Self(Self::get_input_files(&dir))
        }
        pub fn new_number(input: &str, rule: RuleKind) -> Self {
            let dir = format!("{}/{}", input, rule.to_key());
            Self(Self::get_input_files(&dir))
        }

        fn get_input_files(dir: &String) -> Vec<(String, ClassFile)> {
            let path = Path::new(dir);
            fs::read_dir(path)
                .unwrap()
                .map(|x| {
                    let file = String::from(x.unwrap().path().to_str().unwrap());
                    let class_file = parse_file(&file).unwrap();
                    (file, class_file)
                })
                .collect()
        }
    }

    const INPUTS: &str = "tests/inputs/java";

    #[test]
    fn check_no_void_ok() {
        let inputs = LinterInputs::new(INPUTS, RuleKind::CheckNoVoid, true);
        for (file, class_file) in inputs.0 {
            assert!(check_no_void(class_file, file.as_str()).result().is_ok());
        }
    }

    #[test]
    fn check_no_void_fail() {
        let inputs = LinterInputs::new(INPUTS, RuleKind::CheckNoVoid, false);
        for (file, class_file) in inputs.0 {
            assert!(!check_no_void(class_file, file.as_str()).result().is_ok());
        }
    }

    #[test]
    fn parse_ok() {
        let inputs_valid = LinterInputs::new(INPUTS, RuleKind::NoBinaryInNames, true);
        let inputs_invalid = LinterInputs::new(INPUTS, RuleKind::NoBinaryInNames, false);

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
        let inputs = LinterInputs::new(INPUTS, RuleKind::NoBinaryInNames, true);
        for (file, class_file) in inputs.0 {
            assert!(no_binary_in_names(class_file, file.as_str())
                .result()
                .is_ok());
        }
    }

    #[test]
    fn no_binary_in_names_fail() {
        let inputs = LinterInputs::new(INPUTS, RuleKind::NoBinaryInNames, false);
        for (file, class_file) in inputs.0 {
            assert!(!no_binary_in_names(class_file, file.as_str())
                .result()
                .is_ok());
        }
    }

    #[rstest]
    #[case(0, false)]
    #[case(2, false)]
    #[case(3, false)]
    #[case(4, true)]
    #[case(5, true)]
    fn too_many_arguments_test(#[case] max_arguments: u8, #[case] expected: bool) {
        let inputs = LinterInputs::new_number(INPUTS, RuleKind::TooManyArguments);
        inputs.0.iter().for_each(|(file, class_file)| {
            assert!(
                too_many_arguments(class_file.to_owned(), file.as_str(), max_arguments)
                    .result()
                    .is_ok()
                    == expected
            );
        })
    }
}

