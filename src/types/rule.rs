use crate::errors::fail::*;
use crate::rules::java::rules::*;
use classfile_parser::ClassFile;
use colored::Colorize;
use std::fmt;
use std::slice::Iter;
use toml::{Table, Value};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RuleKind {
    NoBinaryInNames,
    TooManyArguments,
    CheckNoVoid,
}

impl RuleKind {
    pub fn to_key(&self) -> &str {
        match *self {
            RuleKind::NoBinaryInNames => "no_binary_in_names",
            RuleKind::TooManyArguments => "too_many_arguments",
            RuleKind::CheckNoVoid => "check_no_void",
        }
    }

    pub fn iterator() -> Iter<'static, RuleKind> {
        static RULE_KIND: [RuleKind; 3] = [
            RuleKind::NoBinaryInNames,
            RuleKind::TooManyArguments,
            RuleKind::CheckNoVoid,
        ];
        RULE_KIND.iter()
    }
}

impl fmt::Display for RuleKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuleKind::NoBinaryInNames => write!(f, "No \"And\" or \"Or\" in Method Names"),
            RuleKind::TooManyArguments => write!(f, "Too Many Arguments"),
            RuleKind::CheckNoVoid => write!(f, "No Void Return"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Rule {
    kind: RuleKind,
    parameter: u8,
}

impl Rule {
    pub fn new(table: &Table, rule_key: &str) -> Option<Self> {
        match rule_key {
            "check_no_void" => Self::select_rule(table.get(rule_key), RuleKind::CheckNoVoid),
            "no_binary_in_names" => {
                Self::select_rule(table.get(rule_key), RuleKind::NoBinaryInNames)
            }
            "too_many_arguments" => {
                Self::select_rule(table.get(rule_key), RuleKind::TooManyArguments)
            }
            _ => None,
        }
    }

    #[cfg(test)]
    pub fn kind(self) -> RuleKind {
        self.kind
    }

    #[cfg(test)]
    pub fn parameter(self) -> u8 {
        self.parameter
    }

    fn select_rule(value: Option<&Value>, kind: RuleKind) -> Option<Self> {
        let Some(value) = value else {
            return None;
        };

        match value {
            Value::Boolean(flag) => {
                if *flag {
                    Some(Self {
                        kind,
                        parameter: 0,
                    })
                } else {
                    None
                }
            }
            Value::Integer(int) => {
                if *int > 0 {
                    Some(Self {
                        kind,
                        parameter: *int as u8,
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn run(&self, class_file: &ClassFile, file: &str) -> RuleResult {
        match self.kind {
            RuleKind::CheckNoVoid => check_no_void(class_file.to_owned(), file),
            RuleKind::NoBinaryInNames => no_binary_in_names(class_file.to_owned(), file),
            RuleKind::TooManyArguments => too_many_arguments(class_file.to_owned(), file, self.parameter),
        }
    }
}

#[derive(Debug)]
pub struct RuleResult {
    file: String,
    rule: RuleKind,
    result: Result<(), Vec<Fail>>,
}

impl RuleResult {
    pub fn new(file: String, rule: RuleKind, result: Result<(), Vec<Fail>>) -> Self {
        Self { file, rule, result }
    }

    pub fn file(&self) -> &String {
        &self.file
    }

    pub fn result(&self) -> &Result<(), Vec<Fail>> {
        &self.result
    }

    pub fn rule(&self) -> RuleKind {
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
                write!(f, "{} {}, Rule: {}", ok, file, rule)
            }
            Err(fails) => {
                let ok = format!("{}", "[FAIL]".red().bold());
                fails.iter().fold(Ok(()), |result, fail| {
                    result.and_then(|_| writeln!(f, "{} {}, Rule: {}, {}", ok, file, rule, fail))
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use toml::Table;

    #[test]
    fn test_rule_kind_iter() {
        static RULE_KIND: [RuleKind; 3] = [
            RuleKind::NoBinaryInNames,
            RuleKind::TooManyArguments,
            RuleKind::CheckNoVoid,
        ];
        let expected = RULE_KIND.iter();
        assert_eq!(RuleKind::iterator().eq(expected), true);
    }

    #[rstest]
    #[case::check_no_void("check_no_void = true", "check_no_void", 0)]
    #[case::no_binary_in_names("no_binary_in_names = true", "no_binary_in_names", 0)]
    #[case::too_many_arguments("too_many_arguments = 4", "too_many_arguments", 4)]
    #[case::no_rule("no_rule = true", "no_rule", 0)]
    fn test_new_rule(#[case] toml: &str, #[case] key: &str, #[case] parameter: u8) {
        let table = toml.parse::<Table>().unwrap();
        let rule = Rule::new(&table, key);
        match rule {
            Some(rule) => {
                assert!(RuleKind::iterator().any(|item| item.eq(&rule.kind())));
                assert_eq!(rule.parameter(), parameter);
            },
            None => assert!(!RuleKind::iterator().any(|item| key == item.to_key())),
        }
    }
}

