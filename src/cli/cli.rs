use classfile_parser::ClassFile;

use crate::rules::java::parse::parse_file;
use crate::config::config::*;
use crate::types::rule::*;

pub fn lint_files(files: Vec<String>, rule: &Rule) {
    let class_files: Vec<(ClassFile, &String)> = files
        .iter()
        .filter_map(|file| match parse_file(&file) {
            Ok(class_file) => Some((class_file, file)),
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        })
        .collect();

    class_files.iter().for_each(|class_file| {
        let result = rule.run(&class_file.0, class_file.1);
        println!("{}", result);
    });
}

pub fn create_rules_list(config_file: String) -> Option<Vec<Rule>> {
    let table = match read_config(config_file) {
        Ok(table) => table,
        Err(e) => { eprintln!("{}", e); return None; }
    };

    select_rules(table)
}

