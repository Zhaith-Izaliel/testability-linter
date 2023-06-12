use classfile_parser::constant_info::{ConstantInfo, Utf8Constant};

use crate::errors::{fail::Fail, generic::*};

/// Extract the utf8 constant from a constant pool with its index
///
/// * `constant_pool`: The constant pool to get the Utf8Constant from
/// * `index`: The index of the Utf8Constant in the constant pool
pub fn extract_utf8_constant(
    constant_pool: &Vec<ConstantInfo>,
    index: u16,
) -> Result<&Utf8Constant, IError> {
    match constant_pool.get((index - 1) as usize) {
        Some(constant) => match constant {
            ConstantInfo::Utf8(constant) => Ok(constant),
            _ => Err(IError::new(
                GenericErrorKind::InvalidFormat,
                String::from("Not an Utf8Constant."),
            )),
        },
        None => {
            return Err(IError::new(
                GenericErrorKind::NotFound,
                String::from("Index out of bound for constant pool."),
            ))
        }
    }
}

pub fn extract_method_name(
    constant_pool: &Vec<ConstantInfo>,
    index: u16,
) -> Result<&String, Fail> {
    match extract_utf8_constant(constant_pool, index) {
        Ok(name) => Ok(&name.utf8_string),
        Err(e) => Err(Fail::new(
            String::from("N/A"),
            e.message().clone(),
            e.kind(),
        )),
    }
}

pub fn extract_method_descriptor<'a>(
    constant_pool: &'a Vec<ConstantInfo>,
    index: u16,
    method_name: &'a String,
) -> Result<&'a String, Fail> {
    match extract_utf8_constant(constant_pool, index) {
        Ok(descriptor) => Ok(&descriptor.utf8_string),
        Err(e) => Err(Fail::new(
            String::from(method_name.to_owned()),
            e.message().clone(),
            e.kind(),
        )),
    }
}

