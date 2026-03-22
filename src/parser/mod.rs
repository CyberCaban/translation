#[cfg(test)]
mod tests;

use crate::lexer::{Lexem, LexemKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}:{}", self.message, self.line, self.column)
    }
}

impl std::error::Error for ParseError {}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration {
    Declare { func: String, identifier: String },
    Conclusion { left: Call, right: Vec<Call> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    pub func: String,
    pub args: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Variable(char),
    Identifier(String),
}

pub struct Parser {
    tokens: Vec<Lexem>,
    idx: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Lexem>) -> Self {
        Self { tokens, idx: 0 }
    }

    // начало парсинга
    // S -> D ( ';' D )* EOF
    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut declarations = Vec::new();
        declarations.push(self.parse_declaration()?);

        loop {
            if self.match_kind(&LexemKind::Semicolon) {
                declarations.push(self.parse_declaration()?);
                continue;
            }

            break;
        }

        if !self.is_eof() {
            let token = self.current();
            return Err(ParseError {
                message: "Unexpected token after end of program".to_string(),
                line: token.line,
                column: token.column,
            });
        }

        Ok(Program { declarations })
    }

    // Декларация может быть либо объявлением, либо заключением
    // D -> 'declare' F '(' Identifier ')' | 'conclusion' K ':' '-' K (',' K)*
    fn parse_declaration(&mut self) -> Result<Declaration, ParseError> {
        // 'declare' ветка
        if self.match_kind(&LexemKind::Declare) {
            let func = self.parse_func()?;
            self.expect_kind(&LexemKind::LParen, "Expected '(' after function")?;
            let identifier = self.parse_identifier()?;
            self.expect_kind(&LexemKind::RParen, "Expected ')' after identifier")?;
            return Ok(Declaration::Declare { func, identifier });
        }

        // 'conclusion' ветка
        if self.match_kind(&LexemKind::Conclusion) {
            let left = self.parse_call()?;
            self.expect_kind(&LexemKind::Colon, "Expected ':' after left expression")?;
            self.expect_kind(&LexemKind::Minus, "Expected '-' after ':'")?;
            let mut right = Vec::new();
            right.push(self.parse_call()?);
            while self.match_kind(&LexemKind::Comma) {
                right.push(self.parse_call()?);
            }
            return Ok(Declaration::Conclusion { left, right });
        }

        let token = self.current();
        Err(ParseError {
            message: "Expected 'declare' or 'conclusion'".to_string(),
            line: token.line,
            column: token.column,
        })
    }

    // Парсинг вызова функции
    // K -> F '(' V (',' V)* ')'
    fn parse_call(&mut self) -> Result<Call, ParseError> {
        let func = self.parse_func()?;
        self.expect_kind(&LexemKind::LParen, "Expected '(' after function")?;

        let mut args = Vec::new();
        args.push(self.parse_value()?);

        while self.match_kind(&LexemKind::Comma) {
            args.push(self.parse_value()?);
        }

        self.expect_kind(&LexemKind::RParen, "Expected ')' after arguments")?;
        Ok(Call { func, args })
    }

    // Парсинг значения (идентификатора или переменной)
    // V -> x | y | z | Identifier
    fn parse_value(&mut self) -> Result<Value, ParseError> {
        let word = self.parse_identifier()?;
        if word == "x" || word == "y" || word == "z" {
            Ok(Value::Variable(word.chars().next().unwrap()))
        } else {
            Ok(Value::Identifier(word))
        }
    }

    // Парсинг имени функции (Q, B или A)
    // F -> 'Q' | 'B' | 'A'
    fn parse_func(&mut self) -> Result<String, ParseError> {
        let token = self.current().clone();
        match &token.kind {
            // Тип токена должен быть Word, а его значение должно быть Q, B или A
            LexemKind::Word(w) if w == "Q" || w == "B" || w == "A" => {
                self.idx += 1;
                Ok(w.clone())
            }
            _ => Err(ParseError {
                message: "Expected function name: Q, B or A".to_string(),
                line: token.line,
                column: token.column,
            }),
        }
    }

    // Парсинг идентификатора (любое слово, не являющееся ключевым)
    // Identifier -> Word (кроме declare, conclusion, Q, B, A)
    fn parse_identifier(&mut self) -> Result<String, ParseError> {
        let token = self.current().clone();
        match token.kind {
            LexemKind::Word(w) => {
                self.idx += 1;
                Ok(w)
            }
            _ => Err(ParseError {
                message: "Expected identifier".to_string(),
                line: token.line,
                column: token.column,
            }),
        }
    }

    // Вспомогательная функция для проверки ожидаемого токена
    fn expect_kind(&mut self, expected: &LexemKind, message: &str) -> Result<(), ParseError> {
        if self.match_kind(expected) {
            Ok(())
        } else {
            let token = self.current();
            Err(ParseError {
                message: message.to_string(),
                line: token.line,
                column: token.column,
            })
        }
    }

    // Вспомогательная функция для проверки и потребления ожидаемого токена
    fn match_kind(&mut self, expected: &LexemKind) -> bool {
        if self.current().kind == *expected {
            self.idx += 1;
            true
        } else {
            false
        }
    }

    // Получение текущего токена
    fn current(&self) -> &Lexem {
        // Защита от выхода за пределы массива токенов
        // saturating_sub(1) гарантирует, что индекс не будет меньше 0, а min гарантирует, что индекс не будет больше len - 1
        &self.tokens[self.idx.min(self.tokens.len().saturating_sub(1))]
    }

    fn is_eof(&self) -> bool {
        self.current().kind == LexemKind::Eof
    }
}
