use colored::Colorize;
use super::generic::GenericErrorKind;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Fail {
    method: String,
    message: String,
    kind: GenericErrorKind,
}

impl Fail {
    pub fn new(method: String, message: String, kind: GenericErrorKind) -> Self {
        Self {method, message, kind}
    }

    pub fn method(&self) -> &String {
        &self.method
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn kind(&self) -> GenericErrorKind {
        self.kind
    }
}

impl fmt::Display for Fail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(format!("(method: {}) - error: {}, trace: {}", self.method(), self.kind().to_string().red().bold(), self.message()).as_str())
    }
}