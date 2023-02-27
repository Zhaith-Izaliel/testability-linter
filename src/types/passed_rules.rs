use crate::enums::rules_enum::*;
use std::fmt;

#[derive(Debug)]
pub struct PassedRule {
    file: String,
    rule: Rules,
}

impl PassedRule {
    pub fn new(file: String, rule: Rules) -> Self {
        Self {file, rule}
    }

    pub fn file(&self) -> &String {
        &self.file
    }

    pub fn rule(&self) -> Rules {
        self.rule
    }
}

impl fmt::Display for PassedRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = format!("[file: {}], rule: {} - OK.", self.file(), self.rule());
        f.write_str(string.as_str())
    }
}