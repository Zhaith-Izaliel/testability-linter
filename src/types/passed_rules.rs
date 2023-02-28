use colored::Colorize;

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

    pub fn to_string(&self) -> String {
        let ok = format!("{}", "OK:".green().bold());
        let file = format!("{}", format!("[file: {}]", self.file()).yellow());
        let string = format!("{} {}, Rule: {}", ok, file, self.rule().to_string().purple());
        format!("{}", string)
    }
}

impl fmt::Display for PassedRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}