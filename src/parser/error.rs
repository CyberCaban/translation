#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl ParseError {
    pub fn new(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self {
            message: message.into(),
            line,
            column,
        }
    }

    pub fn at(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self::new(message, line, column)
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.line == 0 && self.column == 0 {
            return write!(f, "error: {}", self.message);
        }

        write!(
            f,
            "error: {}\n  --> {}:{}\n   |",
            self.message, self.line, self.column
        )
    }
}

impl std::error::Error for ParseError {}