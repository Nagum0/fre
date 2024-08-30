use std::{ffi::OsString, fs};

pub fn transform_file_contents(file_path: &OsString, pattern: &String, to: &String) -> String {
    let file_contents = fs::read_to_string(file_path).unwrap();
    file_contents.replace(pattern, to)
}

pub fn edit_file(path: &OsString, content: String) {
    fs::write(path, content).unwrap();
}
