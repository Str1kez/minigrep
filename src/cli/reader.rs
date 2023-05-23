use std::{fs, io};

pub fn get_content(filename: &str) -> io::Result<String> {
    fs::read_to_string(filename)
}
