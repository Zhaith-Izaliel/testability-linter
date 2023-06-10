use toml::Table;
use std::{ path::Path, fs};
use crate::errors::generic::GenericErrorKind;

pub fn read_config(config_file: &Path) -> Result<Table, GenericErrorKind> {
    let Ok(content) = fs::read_to_string(config_file) else {
        return Err(GenericErrorKind::InvalidPath);
    };

    match content.parse::<Table>() {
        Ok(table) => Ok(table),
        Err(_) => Err(GenericErrorKind::ParseError),
    }
}

pub fn select_rules(table: Table) {

}

