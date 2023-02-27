use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Rules {
    CheckNoVoid
}

impl fmt::Display for Rules {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rules::CheckNoVoid => write!(f, "No Void Method Allowed"),
        }
    }
}