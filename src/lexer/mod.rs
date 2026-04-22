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
                _ => {
                    let line = self.line;
                    let column = self.column;
                    let mut word = String::new();
                    while let Some(c2) = self.current_char() {
                        if c2.is_ascii_alphabetic()
                            || matches!(c2, '=' | '!' | '<' | '>' | '.' | '_')
                            || c2.is_ascii_digit()
                        {
                            word.push(c2);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    let kind = match word.as_str() {
                        "=" => LexemKind::Assignment,
                        "==" => LexemKind::Equal,
                        "!=" => LexemKind::NotEqual,
                        "<" => LexemKind::Less,
                        "<=" => LexemKind::LessEqual,
                        ">" => LexemKind::Greater,
                        ">=" => LexemKind::GreaterEqual,
                        "int" => LexemKind::Type(AType::Int),
                        "float" => LexemKind::Type(AType::Float),
                        "if" => LexemKind::If,
                        "then" => LexemKind::Then,
                        "print" => LexemKind::Print,
                        "begin" => LexemKind::Begin,
                        "end" => LexemKind::End,
                        num if num.parse::<i128>().is_ok() => {
                            LexemKind::IntLiteral(num.parse().unwrap())
                        }
                        num if num.parse::<f64>().is_ok() => {
                            LexemKind::FloatLiteral(num.parse().unwrap())
                        }
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
