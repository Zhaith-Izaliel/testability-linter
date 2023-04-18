use classfile_parser::{
    parse_class,
    types::ClassFile,
    constant_info::{ConstantInfo, Utf8Constant},
};
use crate::{
    errors::{
        generic::*, fail::Fail
    },
    enums::rules_enum::*,
    types::rule::*
};

use crate::utils::path::*;

/* --------------------------------- Public --------------------------------- */

pub fn parse_file(path: &String) -> Result<ClassFile, IError> {
    match parse_class(parse_path_as_absolute(path)?.as_str()) {
        Ok(class_file) => Ok(class_file),
        Err(e) => Err(IError::new(GenericErrorKind::ParseError, e)),
    }
}

pub fn check_void(class_file: ClassFile, file: &str) -> RuleResult {
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

            let descriptor = match extract_utf8_constant(const_pool, method.descriptor_index) {
                Ok(descriptor) => descriptor,
                Err(e) => return Some(
                    Fail::new(String::from("N/A"), e.message().clone(), e.kind())
                )
            };

            if name.utf8_string == "<init>" || name.utf8_string == "<clinit>"|| name.utf8_string == "main" {
                return None;
            }

            if &descriptor.utf8_string[descriptor.utf8_string.len() - 1..] == "V" {
                return Some(
                    Fail::new(name.utf8_string.to_owned(), String::from("This method has return type of void"), GenericErrorKind::RuleCheckFailed)
                );
            }

            None
        }).collect();

        return RuleResult::new(
            String::from(file),
            Rules::CheckNoVoid,
            match errors.is_empty() {
                true => Ok(()),
                false => Err(errors)
            }
        );
}

/* --------------------------------- Private -------------------------------- */

fn extract_utf8_constant(constant_pool: &Vec<ConstantInfo>, index: u16) -> Result<&Utf8Constant, IError> {
match constant_pool.get((index - 1) as usize) {
        Some(constant) => match constant {
            ConstantInfo::Utf8(constant) => Ok(constant),
            _ => Err(IError::new(GenericErrorKind::InvalidFormat, String::from("Not an Utf8Constant."))),
        },
        None => return Err(IError::new(GenericErrorKind::NotFound, String::from("Index out of bound for constant pool."))),
    }
}

/* -------------------------------------------------------------------------- */
/*                                  Test Suit                                 */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;
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
        let inputs_valid = LinterInputs::new(INPUTS, Rules::CheckNoVoid, true);
        let inputs_invalid = LinterInputs::new(INPUTS, Rules::CheckNoVoid, false);

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
    fn check_void_ok() {
        let inputs = LinterInputs::new(INPUTS, Rules::CheckNoVoid, true);
        for (file, class_file) in inputs.0 {
            assert!(check_void(class_file, file.as_str()).result().is_ok());
        }
    }

    #[test]
    fn check_void_fail() {
        let inputs = LinterInputs::new(INPUTS, Rules::CheckNoVoid, false);
        for (file, class_file) in inputs.0 {
            assert!(!check_void(class_file, file.as_str()).result().is_ok());
        }
    }
}
