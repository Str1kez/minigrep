use colored::Colorize;

use crate::models::FoundedLine;

pub fn output(result: &[FoundedLine]) {
    if result.is_empty() {
        return;
    }
    for res in result {
        println!("{}:  {}", res.number().to_string().green(), res.line())
    }
}
