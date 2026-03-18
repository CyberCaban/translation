#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexemKind {
    Word(String),
    LParen,
    RParen,
    Semicolon,
    Comma,
    Colon,
    Minus,
    Declare,
    Conclusion,
    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lexem {
    pub kind: LexemKind,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}:{}", self.message, self.line, self.column)
    }
}

impl std::error::Error for LexError {}


pub struct Lexer {
    idx: usize,
    line: usize,
    column: usize,
    chars: Vec<char>,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            idx: 0,
            line: 1,
            column: 1,
            chars: vec![],
        }
    }
    pub fn lex(&mut self, contents: &str) -> Result<Vec<Lexem>, LexError> {
        self.idx = 0;
        self.line = 1;
        self.column = 1;
        self.chars = contents.chars().collect();

        let mut parsed_lexems = Vec::new();

        while let Some(ch) = self.current_char() {
            if ch.is_ascii_whitespace() {
                self.advance();
                continue;
            }

            match ch {
                '(' => {
                    parsed_lexems.push(self.make_lexem(LexemKind::LParen));
                    self.advance();
                }
                ')' => {
                    parsed_lexems.push(self.make_lexem(LexemKind::RParen));
                    self.advance();
                }
                ';' => {
                    parsed_lexems.push(self.make_lexem(LexemKind::Semicolon));
                    self.advance();
                }
                ',' => {
                    parsed_lexems.push(self.make_lexem(LexemKind::Comma));
                    self.advance();
                }
                ':' => {
                    parsed_lexems.push(self.make_lexem(LexemKind::Colon));
                    self.advance();
                }
                '-' => {
                    parsed_lexems.push(self.make_lexem(LexemKind::Minus));
                    self.advance();
                }
                c if c.is_ascii_alphabetic() => {
                    let line = self.line;
                    let column = self.column;
                    let mut word = String::new();
                    while let Some(c2) = self.current_char() {
                        if c2.is_ascii_alphabetic() {
                            word.push(c2);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    let kind = match word.as_str() {
                        "declare" => LexemKind::Declare,
                        "conclusion" => LexemKind::Conclusion,
                        _ => LexemKind::Word(word),
                    };
                    parsed_lexems.push(Lexem { kind, line, column });
                }
                _ => {
                    return Err(LexError {
                        message: format!("Unexpected character '{}'", ch),
                        line: self.line,
                        column: self.column,
                    });
                }
            }
        }

        parsed_lexems.push(Lexem {
            kind: LexemKind::Eof,
            line: self.line,
            column: self.column,
        });

        Ok(parsed_lexems)
    }

    fn current_char(&self) -> Option<char> {
        self.chars.get(self.idx).copied()
    }

    fn advance(&mut self) {
        if let Some(ch) = self.current_char() {
            self.idx += 1;
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
    }

    fn make_lexem(&self, kind: LexemKind) -> Lexem {
        Lexem {
            kind,
            line: self.line,
            column: self.column,
        }
    }
}
