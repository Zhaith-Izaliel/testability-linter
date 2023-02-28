use classfile_parser::{
    parse_class,
    types::ClassFile,
    constant_info::{ConstantInfo, Utf8Constant},
};
use crate::{
    errors::{
        generic::*, failed_rules::FailedRule
    },
    types::passed_rules::PassedRule, enums::rules_enum::Rules
};

use crate::utils::path::*;

/* --------------------------------- Public --------------------------------- */

pub fn parse_file(path: &String) -> Result<ClassFile, IError> {
    match parse_class(parse_path_as_absolute(path)?.as_str()) {
        Ok(class_file) => Ok(class_file),
        Err(e) => Err(IError::new(GenericErrorKind::ParseError, e)),
    }
}

pub fn check_void(class_file: ClassFile, file: &str) -> Result<PassedRule, Vec<FailedRule>> {
    let const_pool = &class_file.const_pool;

    let errors: Vec<FailedRule> = class_file.methods
        .iter()
        .filter_map(|method| {
            let name = match extract_utf8_constant(const_pool, method.name_index - 1) {
                Ok(name) => name,
                Err(e) => return Some(
                    FailedRule::new(String::from(file), Rules::CheckNoVoid, String::from("N/A"), e.message().clone(), e.kind())
                )
            };

            let descriptor = match extract_utf8_constant(const_pool, method.descriptor_index - 1) {
                Ok(descriptor) => descriptor,
                Err(e) => return Some(
                    FailedRule::new(String::from(file), Rules::CheckNoVoid, String::from("N/A"), e.message().clone(), e.kind())
                )
            };

            if name.utf8_string == "<init>" || name.utf8_string == "<clinit>"|| name.utf8_string == "main" {
                return None;
            }

            if &descriptor.utf8_string[descriptor.utf8_string.len() - 1..] == "V" {
                return Some(
                    FailedRule::new(String::from(file), Rules::CheckNoVoid, name.utf8_string.to_owned(), String::from("This method has return type of void"), GenericErrorKind::RuleCheckFailed)
                );
            }

            None
        }).collect();

        match errors.is_empty() {
            true => Ok(PassedRule::new(String::from(file), Rules::CheckNoVoid)),
            false => Err(errors)
        }
}

/* --------------------------------- Private -------------------------------- */

fn extract_utf8_constant(constant_pool: &Vec<ConstantInfo>, index: u16) -> Result<&Utf8Constant, IError> {
    match constant_pool.get(index as usize) {
        Some(constant) => match constant {
            ConstantInfo::Utf8(constant) => Ok(constant),
            _ => Err(IError::new(GenericErrorKind::InvalidFormat, String::from("Not an Utf8Constant."))),
        },
        None => return Err(IError::new(GenericErrorKind::NotFound, String::from("Index out of bound for constant pool."))),
    }
}