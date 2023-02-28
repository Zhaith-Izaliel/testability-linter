use crate::enums::rules_enum::*;
use colored::Colorize;
use super::generic::GenericErrorKind;
use std::fmt;

#[derive(Debug, Clone)]
pub struct FailedRule {
    file: String,
    rule: Rules,
    method: String,
    message: String,
    kind: GenericErrorKind,
}

impl FailedRule {
    pub fn new(file: String, rule: Rules, method: String, message: String, kind: GenericErrorKind) -> Self {
        Self {file, rule, method, message, kind}
    }

    pub fn file(&self) -> &String {
        &self.file
    }

    pub fn rule(&self) -> Rules {
        self.rule
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

    pub fn to_string(&self) -> String {
        let fail = format!("{}", "FAIL:".red().bold());
        let file = format!("{}", format!("[file: {}]", self.file()).yellow());
        let string = format!("{} {},", fail, file);
        match self.kind {
            GenericErrorKind::RuleCheckFailed => {
                let string = format!("{} Rule: {},", string, self.rule().to_string().purple());
                match self.rule() {
                    Rules::CheckNoVoid => format!("{} trace: (method: {}) - {}", string, self.method(), self.message()),
                }
            }
            _ => format!("{} error: {}, trace: {}", string, self.kind(), self.message()),
        }
    }
}

impl fmt::Display for FailedRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}