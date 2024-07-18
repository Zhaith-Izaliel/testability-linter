use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum GenericErrorKind {
    ParseError,
    NotFound,
    InvalidFormat,
    RuleCheckFailed,
    InvalidPath,
    Other,
}

impl fmt::Display for GenericErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GenericErrorKind::ParseError => write!(f, "Parser Error"),
            GenericErrorKind::NotFound => write!(f, "Not Found"),
            GenericErrorKind::InvalidFormat => write!(f, "Invalid Format"),
            GenericErrorKind::InvalidPath => write!(f, "Invalid Path"),
            GenericErrorKind::RuleCheckFailed => write!(f, "Rule Check Failed"),
            GenericErrorKind::Other => write!(f, "Other"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IError {
    kind: GenericErrorKind,
    message: String,
}

impl IError {
    pub fn new(kind: GenericErrorKind, message: String) -> Self {
        Self { kind, message }
    }

    pub fn kind(&self) -> GenericErrorKind {
        self.kind
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

impl fmt::Display for IError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string  = format!("error: {}, trace: {}", self.kind(), self.message());
        f.write_str(string.as_str())
    }
}