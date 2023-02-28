use classfile_parser::ClassFile;

use crate::{rules::java::java_rules::{
    parse_file,
    check_void
}};
use crate::utils::print::*;

pub fn lint_files(files: Vec<String>) {
    let class_files: Vec<(ClassFile, &String)> = files.iter().filter_map(|file| {
        match parse_file(&file) {
            Ok(class_file) => Some((class_file, file)),
            Err(e) => {println!("{}", e); None}
        }
    }).collect();

    class_files.iter().for_each(|class_file| {
        let result = check_void(class_file.0.to_owned(), class_file.1);
        print_lint(result);
    });
}