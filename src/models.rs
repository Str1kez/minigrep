use std::env;
use std::error::Error;

use crate::cli::parse_args;

pub struct FoundedLine<'a> {
    line: &'a str,
    number: usize,
}

impl FoundedLine<'_> {
    pub fn new(line: &str, number: usize) -> FoundedLine {
        FoundedLine { line, number }
    }
    pub fn line(&self) -> &str {
        self.line
    }
    pub fn number(&self) -> usize {
        self.number
    }
}

pub struct Config<'a> {
    query: &'a str,
    filename: &'a str,
    sensetive: bool,
}

impl Config<'_> {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        let (query, filename) = parse_args(args)?;
        let sensetive = env::var("CASE_INSENSETIVE").unwrap_or_default().is_empty();
        Ok(Config {
            query,
            filename,
            sensetive,
        })
    }
    pub fn query(&self) -> &str {
        self.query
    }
    pub fn filename(&self) -> &str {
        self.filename
    }
    pub fn sensetive(&self) -> bool {
        self.sensetive
    }
}