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
