use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Rules {
    NoBinaryInNames,
    TooManyArguments,
    CheckNoVoid,
}

impl Rules {
    #[cfg(test)]
    pub fn to_dir_string(&self) -> &str {
        match *self {
            Rules::NoBinaryInNames => "no_binary_in_names",
            Rules::TooManyArguments => "too_many_arguments",
            Rules::CheckNoVoid => "check_no_void",
        }
    }
}

impl fmt::Display for Rules {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rules::NoBinaryInNames => write!(f, "No \"And\" or \"Or\" in Method Names"),
            Rules::TooManyArguments => write!(f, "Too Many Arguments"),
            Rules::CheckNoVoid => write!(f, "This method has a Void return type"),
        }
    }
}

