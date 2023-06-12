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
            false => Err(errors),
        },
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

    const INPUTS: &str = "tests/inputs/java";

    fn parse_file_for_test(file: &str) -> (ClassFile, String) {
        let file = format!("{}/{}", INPUTS, file);
        (parse_file(&file).unwrap(), String::from(file).to_owned() )
    }

    #[rstest]
    #[case("check_no_void/invalid/Example.class", false)]
    #[case("check_no_void/valid/Test.class", true)]
    fn test_check_no_void(#[case] file: &str, #[case] expected: bool) {
        let class_and_file = parse_file_for_test(file);
        let result = check_no_void(class_and_file.0, class_and_file.1.as_str());
        assert_eq!(result.result().is_ok(), expected);
    }

    #[rstest]
    #[case::invalid_and_camel("no_binary_in_names/invalid/AndInNameCamelCase.class", false)]
    #[case::invalid_or_camel("no_binary_in_names/invalid/OrInNameCamelCase.class", false)]
    #[case::invalid_and_camel_start("no_binary_in_names/invalid/AndInNameCamelCaseStart.class", false)]
    #[case::invalid_or_camel_start("no_binary_in_names/invalid/OrInNameCamelCaseStart.class", false)]
    #[case::invalid_and_snake("no_binary_in_names/invalid/AndInNameSnakeCase.class", false)]
    #[case::invalid_or_snake("no_binary_in_names/invalid/OrInNameSnakeCase.class", false)]
    #[case::invalid_and_snake_upper("no_binary_in_names/invalid/AndInNameSnakeCaseUpper.class", false)]
    #[case::invalid_or_snake_upper("no_binary_in_names/invalid/OrInNameSnakeCaseUpper.class", false)]
    #[case::invalid_and_snake_upper_start("no_binary_in_names/invalid/AndInNameSnakeCaseUpperStart.class", false)]
    #[case::invalid_or_snake_upper_start("no_binary_in_names/invalid/AndInNameSnakeCaseUpperStart.class", false)]
    #[case::valid_lower_and("no_binary_in_names/valid/LowerAndNameOnly.class", true)]
    #[case::valid_lower_or("no_binary_in_names/valid/LowerOrNameOnly.class", true)]
    #[case::valid_upper_and("no_binary_in_names/valid/UpperAndNameOnly.class", true)]
    #[case::valid_upper_or("no_binary_in_names/valid/UpperOrNameOnly.class", true)]
    #[case::valid_camel_and("no_binary_in_names/valid/CamelAndNameOnly.class", true)]
    #[case::valid_camel_or("no_binary_in_names/valid/CamelOrNameOnly.class", true)]
    fn test_no_binary_in_names(#[case] file: &str, #[case] expected: bool) {
        let class_and_file = parse_file_for_test(file);
        let result = no_binary_in_names(class_and_file.0, class_and_file.1.as_str());
        assert_eq!(result.result().is_ok(), expected);
    }

    #[rstest]
    #[case(0, false)]
    #[case(1, false)]
    #[case(2, false)]
    #[case(3, false)]
    #[case(4, true)]
    #[case(5, true)]
    #[case(6, true)]
    #[case(7, true)]
    #[case(8, true)]
    #[case(9, true)]
    #[case(10, true)]
    fn test_too_many_arguments(#[case] max_arguments: u8, #[case] expected: bool) {
        let class_and_file = parse_file_for_test("too_many_arguments/TooManyArguments.class");
        let result = too_many_arguments(class_and_file.0, class_and_file.1.as_str(), max_arguments);
        assert_eq!(result.result().is_ok(), expected);
    }

    #[rstest]
    #[case::zero_argument(0, 5)]
    #[case::one_argument(1, 4)]
    #[case::two_arguments(2, 3)]
    #[case::three_arguments(3, 2)]
    #[case::four_arguments(4, 1)]
    #[case::five_arguments(5, 0)]
    fn test_too_many_arguments_number(#[case] max_arguments: u8, #[case] expected_errors: usize) {
        let class_and_file = parse_file_for_test("too_many_arguments/TooManyArgumentsNumber.class");
        let result = too_many_arguments(class_and_file.0, class_and_file.1.as_str(), max_arguments);
        match result.result()  {
            Ok(_) => assert_eq!(0, expected_errors),
            Err(vector) => assert_eq!(vector.len(), expected_errors),
        }
    }
}

