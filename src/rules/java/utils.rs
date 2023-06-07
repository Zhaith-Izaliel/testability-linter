use classfile_parser::constant_info::{
    ConstantInfo,
    Utf8Constant,
};

use crate::errors::generic::*;

/// Extract the utf8 constant from a constant pool with its index
///
/// * `constant_pool`: The constant pool to get the Utf8Constant from
/// * `index`: The index of the Utf8Constant in the constant pool
pub fn extract_utf8_constant(constant_pool: &Vec<ConstantInfo>, index: u16)
-> Result<&Utf8Constant, IError> {
    match constant_pool.get((index - 1) as usize) {
        Some(constant) => match constant {
            ConstantInfo::Utf8(constant) => Ok(constant),
            _ => Err(IError::new(
                GenericErrorKind::InvalidFormat,
                String::from("Not an Utf8Constant."))),
        },
        None => return Err(IError::new(GenericErrorKind::NotFound,
            String::from("Index out of bound for constant pool."))),
    }
}

