use classfile_parser::ClassFile;

use crate::rules::java::parse::parse_file;
use crate::rules::java::rules::*;

pub fn lint_files(files: Vec<String>) {
    let class_files: Vec<(ClassFile, &String)> = files
        .iter()
        .filter_map(|file| match parse_file(&file) {
            Ok(class_file) => Some((class_file, file)),
            Err(e) => {
                println!("{}", e);
                None
            }
        })
        .collect();

    class_files.iter().for_each(|class_file| {
        let result = too_many_arguments(class_file.0.to_owned(), class_file.1, 10);
        println!("{}", result);
    });
}

