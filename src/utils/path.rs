use std::fs::canonicalize;
use crate::errors::generic::{
    GenericErrorKind,
    IError,
};


pub fn parse_path_as_absolute(path: &String) -> Result<String, IError> {
    let path = match canonicalize(path) {
        Ok(path) => path,
        Err(e) => return Err(IError::new(GenericErrorKind::InvalidPath, format!("{:?}", e))),
    };
    match path.with_extension("").to_str() {
        Some(s) => Ok(String::from(s)),
        None => Err(IError::new(
            GenericErrorKind::Other,
            format!("Failed to get file stem from {:?}", path)
        ))
    }
}