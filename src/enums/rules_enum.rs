use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Rules {
    CheckNoVoid
}

impl Rules {
    #[cfg(test)]
    pub fn to_dir_string(&self) -> &str {
        match *self {
            Rules::CheckNoVoid => "check_no_void"
        }
    }
}

impl fmt::Display for Rules {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rules::CheckNoVoid => write!(f,"No Void Method Allowed"),
        }
    }
}