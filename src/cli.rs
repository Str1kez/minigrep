use colored::Colorize;
use std::{fs, io};

use crate::models::FoundedLine;

pub fn parse_args(args: &[String]) -> Result<(&str, &str), &'static str> {
    if args.len() < 3 {
        return Err("Args not enough");
    }
    Ok((&args[1], &args[2]))
}

pub fn get_content(filename: &str) -> io::Result<String> {
    fs::read_to_string(filename)
}

pub fn output(result: &[FoundedLine]) {
    if result.is_empty() {
        return;
    }
    for res in result {
        println!("{}:  {}", res.number().to_string().green(), res.line())
    }
}
