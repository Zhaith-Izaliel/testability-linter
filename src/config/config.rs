use toml::Table;
use std::{ path::Path, fs};
use crate::errors::generic::GenericErrorKind;
use crate::types::rule::{ RuleKind, Rule };

pub fn read_config(config_file: &Path) -> Result<Table, GenericErrorKind> {
    let Ok(content) = fs::read_to_string(config_file) else {
        return Err(GenericErrorKind::InvalidPath);
    };

    match content.parse::<Table>() {
        Ok(table) => Ok(table),
        Err(_) => Err(GenericErrorKind::ParseError),
    }
}

pub fn select_rules(table: Table) -> Option<Vec<Rule>> {
    let mut vector: Vec<Rule> = Vec::new();

    RuleKind::iterator().for_each(|kind| {
        let rule = Rule::new(&table, kind.to_key());
        match rule {
            Some(rule) => vector.push(rule),
            None => (),
        }
    });

    if vector.is_empty() {
        return None;
    }

    Some(vector)
}

