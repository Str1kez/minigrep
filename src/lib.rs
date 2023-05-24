mod cli;
mod models;

use cli::{get_content, output};
pub use models::Config;
use models::FoundedLine;
use std::error::Error;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let content = get_content(config.filename())?;
    let result = if config.sensetive() {
        find_query(config.query(), &content)
    } else {
        find_query_insensetive(config.query(), &content)
    };
    output(&result);
    Ok(())
}

fn find_query_insensetive<'a>(query: &str, content: &'a str) -> Vec<FoundedLine<'a>> {
    content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.to_lowercase().contains(&query.to_lowercase()))
        .map(|(number, line)| FoundedLine::new(line, number.wrapping_add(1)))
        .collect()
}

fn find_query<'a>(query: &str, content: &'a str) -> Vec<FoundedLine<'a>> {
    content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(query))
        .map(|(number, line)| FoundedLine::new(line, number.wrapping_add(1)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_query_correct() {
        let content = get_content("test.txt").expect("Error in get_content function");
        let result = find_query("dreary", &content);
        assert_eq!(result[0].line(), "How dreary to be somebody!")
    }
}
