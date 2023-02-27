use crate::{types::passed_rules::PassedRule, errors::failed_rules::FailedRule};

pub fn print_lint(result: Result<PassedRule, Vec<FailedRule>>) {
    match result {
        Ok(passed_rule) => println!("{}", passed_rule),
        Err(failed_rules) => failed_rules.iter().for_each(|fail| println!("{}", fail)),
    }
}