use std::env;
use std::error::Error;

use crate::cli::parse_args;

pub struct Config {
    query: String,
    filename: String,
    sensetive: bool,
}

impl Config {
    pub fn new(args: env::Args) -> Result<Config, Box<dyn Error>> {
        let (query, filename) = parse_args(args)?;
        let sensetive = env::var("CASE_INSENSETIVE").unwrap_or_default().is_empty();
        Ok(Config {
            query,
            filename,
            sensetive,
        })
    }
    pub fn query(&self) -> &str {
        &self.query
    }
    pub fn filename(&self) -> &str {
        &self.filename
    }
    pub fn sensetive(&self) -> bool {
        self.sensetive
    }
}
