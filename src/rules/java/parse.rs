use classfile_parser::{
    parse_class,
    types::ClassFile,
};
use crate::errors::generic::*;
use crate::utils::path::*;

/// Parse a file using classfile parser, returning a result containing it.
///
/// * `path`: the path of the classfile
///
pub fn parse_file(path: &String) -> Result<ClassFile, IError> {
    match parse_class(parse_path_as_absolute(path)?.as_str()) {
        Ok(class_file) => Ok(class_file),
        Err(e) => Err(IError::new(GenericErrorKind::ParseError, e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::empty_file("", false)]
    #[case("tests/inputs/java/check_no_void/invalid/Example.class", true)]
    #[case("tests/inputs/java/check_no_void/valid/Test.class", true)]
    #[case::invalid_and_camel("tests/inputs/java/no_binary_in_names/invalid/AndInNameCamelCase.class", true)]
    #[case::invalid_or_camel("tests/inputs/java/no_binary_in_names/invalid/OrInNameCamelCase.class", true)]
    #[case::invalid_and_camel_start("tests/inputs/java/no_binary_in_names/invalid/AndInNameCamelCaseStart.class", true)]
    #[case::invalid_or_camel_start("tests/inputs/java/no_binary_in_names/invalid/OrInNameCamelCaseStart.class", true)]
    #[case::invalid_and_snake("tests/inputs/java/no_binary_in_names/invalid/AndInNameSnakeCase.class", true)]
    #[case::invalid_or_snake("tests/inputs/java/no_binary_in_names/invalid/OrInNameSnakeCase.class", true)]
    #[case::invalid_and_snake_upper("tests/inputs/java/no_binary_in_names/invalid/AndInNameSnakeCaseUpper.class", true)]
    #[case::invalid_or_snake_upper("tests/inputs/java/no_binary_in_names/invalid/OrInNameSnakeCaseUpper.class", true)]
    #[case::invalid_and_snake_upper_start("tests/inputs/java/no_binary_in_names/invalid/AndInNameSnakeCaseUpperStart.class", true)]
    #[case::invalid_or_snake_upper_start("tests/inputs/java/no_binary_in_names/invalid/AndInNameSnakeCaseUpperStart.class", true)]
    #[case::valid_lower_and("tests/inputs/java/no_binary_in_names/valid/LowerAndNameOnly.class", true)]
    #[case::valid_lower_or("tests/inputs/java/no_binary_in_names/valid/LowerOrNameOnly.class", true)]
    #[case::valid_upper_and("tests/inputs/java/no_binary_in_names/valid/UpperAndNameOnly.class", true)]
    #[case::valid_upper_or("tests/inputs/java/no_binary_in_names/valid/UpperOrNameOnly.class", true)]
    #[case::valid_camel_and("tests/inputs/java/no_binary_in_names/valid/CamelAndNameOnly.class", true)]
    #[case::valid_camel_or("tests/inputs/java/no_binary_in_names/valid/CamelOrNameOnly.class", true)]
    #[case::too_many_arguments("tests/inputs/java/too_many_arguments/TooManyArguments.class", true)]
    fn test_parse(#[case] file: &str, #[case] expected: bool) {
        let result = parse_file(&String::from(file));
        assert_eq!(result.is_ok(), expected)
    }
}

