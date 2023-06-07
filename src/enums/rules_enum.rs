use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Rules {
    NoBinaryInNames
}

impl Rules {
    #[cfg(test)]
    pub fn to_dir_string(&self) -> &str {
        match *self {
            Rules::NoBinaryInNames => "no_binary_in_names"
        }
    }
}

impl fmt::Display for Rules {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rules::NoBinaryInNames =>
                write!(f,"No \"And\" or \"Or\" in Method Names"),
        }
    }
}

