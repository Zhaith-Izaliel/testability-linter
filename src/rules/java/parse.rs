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

