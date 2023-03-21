use colored::Colorize;
use crate::errors::fail::*;
use crate::enums::rules_enum::*;
use std::fmt;

#[derive(Debug)]
pub struct RuleResult {
    file: String,
    rule: Rules,
    result: Result<(), Vec<Fail>>,
}

impl RuleResult {
    pub fn new(file: String, rule: Rules, result: Result<(),Vec<Fail>>) -> Self {
        Self {file, rule, result}
    }

    pub fn file(&self) -> &String {
        &self.file
    }

    pub fn result(&self) -> &Result<(),Vec<Fail>> {
        &self.result
    }

    pub fn rule(&self) -> Rules {
        self.rule
    }
}

impl fmt::Display for RuleResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let file = format!("{}", format!("(file: {})", self.file()).yellow());
        let rule = format!("{}", self.rule().to_string().purple());
        match self.result() {
            Ok(()) => {
                let ok = format!("{}", "[OK]".green().bold());
                write!(f, "{} {}, Rules: {}", ok, file, rule)
            }
            Err(fails) => {
                let ok = format!("{}", "[FAIL]".red().bold());
                fails.iter().fold(Ok(()), |result, fail| {
                    result.and_then(|_| writeln!(f, "{} {}, Rules: {}, {}", ok, file, rule, fail))
                })
            }
        }
    }
}