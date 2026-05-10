#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AType {
    Int,
    Float,
    UserDefined(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexemKind {
    Word(String),
    LParen,
    RParen,
    Semicolon,
    Comma,
    Colon,
    Type(AType),
    // Statement operators
    Assignment,
    If,
    Then,
    Print,
    Begin,
    End,
    // Comparison operators
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    // Math operators
    Plus,
    Minus,
    Multiply,
    Divide,
    // Numbers
    IntLiteral(i128),
    FloatLiteral(f64),
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
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

            let line = self.line;
            let column = self.column;

            if ch.is_ascii_alphabetic() || ch == '_' {
                let word = self.consume_while(|c| c.is_ascii_alphanumeric() || c == '_');
                let kind = match word.as_str() {
                    "int" => LexemKind::Type(AType::Int),
                    "float" => LexemKind::Type(AType::Float),
                    "if" => LexemKind::If,
                    "then" => LexemKind::Then,
                    "print" => LexemKind::Print,
                    "begin" => LexemKind::Begin,
                    "end" => LexemKind::End,
                    _ => LexemKind::Word(word),
                };
                parsed_lexems.push(Lexem { kind, line, column });
                continue;
            }

            if ch.is_ascii_digit() {
                let number = self.consume_number();
                let kind = match number.parse::<i128>() {
                    Ok(value) => LexemKind::IntLiteral(value),
                    Err(_) => LexemKind::FloatLiteral(number.parse().unwrap()),
                };
                parsed_lexems.push(Lexem { kind, line, column });
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
                '+' => {
                    parsed_lexems.push(self.make_lexem(LexemKind::Plus));
                    self.advance();
                }
                '*' => {
                    parsed_lexems.push(self.make_lexem(LexemKind::Multiply));
                    self.advance();
                }
                '/' => {
                    parsed_lexems.push(self.make_lexem(LexemKind::Divide));
                    self.advance();
                }
                '=' => {
                    self.advance();
                    if self.current_char() == Some('=') {
                        self.advance();
                        parsed_lexems.push(Lexem {
                            kind: LexemKind::Equal,
                            line,
                            column,
                        });
                    } else {
                        parsed_lexems.push(Lexem {
                            kind: LexemKind::Assignment,
                            line,
                            column,
                        });
                    }
                }
                '!' => {
                    self.advance();
                    if self.current_char() == Some('=') {
                        self.advance();
                        parsed_lexems.push(Lexem {
                            kind: LexemKind::NotEqual,
                            line,
                            column,
                        });
                    } else {
                        return Err(LexError {
                            message: format!("Unexpected character '{}'", '!'),
                            line,
                            column,
                        });
                    }
                }
                '<' => {
                    self.advance();
                    if self.current_char() == Some('=') {
                        self.advance();
                        parsed_lexems.push(Lexem {
                            kind: LexemKind::LessEqual,
                            line,
                            column,
                        });
                    } else {
                        parsed_lexems.push(Lexem {
                            kind: LexemKind::Less,
                            line,
                            column,
                        });
                    }
                }
                '>' => {
                    self.advance();
                    if self.current_char() == Some('=') {
                        self.advance();
                        parsed_lexems.push(Lexem {
                            kind: LexemKind::GreaterEqual,
                            line,
                            column,
                        });
                    } else {
                        parsed_lexems.push(Lexem {
                            kind: LexemKind::Greater,
                            line,
                            column,
                        });
                    }
                }
                _ => {
                    return Err(LexError {
                        message: format!("Unexpected character '{}'", ch),
                        line,
                        column,
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

    fn consume_while<F>(&mut self, mut predicate: F) -> String
    where
        F: FnMut(char) -> bool,
    {
        let mut text = String::new();

        while let Some(ch) = self.current_char() {
            if predicate(ch) {
                text.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        text
    }

    fn consume_number(&mut self) -> String {
        let mut number = self.consume_while(|ch| ch.is_ascii_digit());

        if self.current_char() == Some('.') {
            number.push('.');
            self.advance();
            number.push_str(&self.consume_while(|ch| ch.is_ascii_digit()));
        }

        number
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
