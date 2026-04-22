#[cfg(test)]
mod tests;

use crate::lexer::{AType, Lexem, LexemKind};

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
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub vars_decls: Vec<VarsDecl>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarsDecl {
    Var { ttype: AType, name: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Assignment {
        var: String,
        value: Expr,
    },
    If {
        condition: Comparison,
        then_branch: Vec<Statement>,
    },
    Print {
        value: Expr,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Comparison {
    Equal(Expr, Expr),
    NotEqual(Expr, Expr),
    Less(Expr, Expr),
    LessEqual(Expr, Expr),
    Greater(Expr, Expr),
    GreaterEqual(Expr, Expr),
    Expression(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Plus { left: Box<Expr>, term: Term },
    Minus { left: Box<Expr>, term: Term },
    Term(Term),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Multiply { left: Box<Term>, factor: Factor },
    Divide { left: Box<Term>, factor: Factor },
    Factor(Factor),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Factor {
    Variable(String),
    IntLiteral(i128),
    FloatLiteral(f64),
    Paren(Box<Expr>),
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
    // Program -> VarsDecls Body
    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        // VarsDecls branch
        let mut declarations = Vec::new();
        declarations.push(self.parse_declaration()?);

        loop {
            if self.match_kind(&LexemKind::Semicolon) {
                declarations.push(self.parse_declaration()?);
                continue;
            }

            break;
        }

        // Body branch
        let body = self.parse_body()?;

        if !self.is_eof() {
            let token = self.current();
            return Err(ParseError {
                message: "Unexpected token after end of program".to_string(),
                line: token.line,
                column: token.column,
            });
        }

        Ok(Program {
            vars_decls: declarations,
            body,
        })
    }

    // Декларация может быть либо объявлением, либо заключением
    // VarsDecls -> {VarsDecl ';'}
    // VarsDecl -> Type Identifier {, Identifier}
    fn parse_declaration(&mut self) -> Result<VarsDecl, ParseError> {
        let kind = self.current_kind().clone();
        if let LexemKind::Type(atype) = kind {
            self.idx += 1;
            let identifier = self.parse_identifier()?;
            return Ok(VarsDecl::Var {
                ttype: atype,
                name: identifier,
            });
        }

        let token = self.current();
        Err(ParseError {
            message: "Expected variable declaration".to_string(),
            line: token.line,
            column: token.column,
        })
    }

    fn parse_body(&mut self) -> Result<Vec<Statement>, ParseError> {
        self.expect_kind(&LexemKind::Begin, "No begin keyword")?;
        let mut statements = Vec::new();

        while !self.is_eof() {
            statements.push(self.parse_statement()?);
            if !self.match_kind(&LexemKind::Semicolon) {
                break;
            }
        }
        self.expect_kind(&LexemKind::End, "No end keyword")?;

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        let kind = self.current_kind().clone();
        match kind {
            LexemKind::Word(_) => self.parse_assignment(),
            LexemKind::If => self.parse_if(),
            LexemKind::Print => self.parse_print(),
            _ => {
                let token = self.current();
                Err(ParseError {
                    message: "Expected statement".to_string(),
                    line: token.line,
                    column: token.column,
                })
            }
        }
    }

    fn parse_assignment(&mut self) -> Result<Statement, ParseError> {
        let var = self.parse_identifier()?;
        self.expect_kind(&LexemKind::Assignment, "Expected assignment operator")?;
        let value = self.parse_expr()?;
        Ok(Statement::Assignment { var, value })
    }

    fn parse_if(&mut self) -> Result<Statement, ParseError> {
        self.expect_kind(&LexemKind::If, "Expected if keyword")?;
        let condition = self.parse_comparison()?;
        self.expect_kind(&LexemKind::Then, "Expected then keyword")?;
        let then_branch = self.parse_body()?;
        Ok(Statement::If {
            condition,
            then_branch,
        })
    }

    fn parse_print(&mut self) -> Result<Statement, ParseError> {
        self.expect_kind(&LexemKind::Print, "Expected print keyword")?;
        let value = self.parse_expr()?;
        Ok(Statement::Print { value })
    }

    fn parse_comparison(&mut self) -> Result<Comparison, ParseError> {
        let left = self.parse_expr()?;

        let kind = self.current_kind().clone();
        let comparison = match kind {
            LexemKind::Equal => {
                self.idx += 1;
                Comparison::Equal(left, self.parse_expr()?)
            }
            LexemKind::NotEqual => {
                self.idx += 1;
                Comparison::NotEqual(left, self.parse_expr()?)
            }
            LexemKind::Less => {
                self.idx += 1;
                Comparison::Less(left, self.parse_expr()?)
            }
            LexemKind::LessEqual => {
                self.idx += 1;
                Comparison::LessEqual(left, self.parse_expr()?)
            }
            LexemKind::Greater => {
                self.idx += 1;
                Comparison::Greater(left, self.parse_expr()?)
            }
            LexemKind::GreaterEqual => {
                self.idx += 1;
                Comparison::GreaterEqual(left, self.parse_expr()?)
            }
            _ => return Ok(Comparison::Expression(left)),
        };
        Ok(comparison)
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let mut expr = Expr::Term(self.parse_term()?);

        while let LexemKind::Plus | LexemKind::Minus = self.current_kind() {
            let op = self.current_kind().clone();
            self.idx += 1;
            let term = self.parse_term()?;
            expr = match op {
                LexemKind::Plus => Expr::Plus {
                    left: Box::new(expr),
                    term,
                },
                LexemKind::Minus => Expr::Minus {
                    left: Box::new(expr),
                    term,
                },
                _ => unreachable!(),
            };
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Term, ParseError> {
        let mut term = Term::Factor(self.parse_factor()?);

        while let LexemKind::Multiply | LexemKind::Divide = self.current_kind() {
            let op = self.current_kind().clone();
            self.idx += 1;
            let factor = self.parse_factor()?;
            term = match op {
                LexemKind::Multiply => Term::Multiply {
                    left: Box::new(term),
                    factor,
                },
                LexemKind::Divide => Term::Divide {
                    left: Box::new(term),
                    factor,
                },
                _ => unreachable!(),
            };
        }

        Ok(term)
    }

    fn parse_factor(&mut self) -> Result<Factor, ParseError> {
        let kind = self.current_kind().clone();
        match kind {
            LexemKind::Word(_) => {
                let var = self.parse_identifier()?;
                Ok(Factor::Variable(var))
            }
            LexemKind::IntLiteral(n) => {
                self.idx += 1;
                Ok(Factor::IntLiteral(n))
            }
            LexemKind::FloatLiteral(f) => {
                self.idx += 1;
                Ok(Factor::FloatLiteral(f))
            }
            LexemKind::LParen => {
                self.idx += 1;
                let expr = self.parse_expr()?;
                self.expect_kind(&LexemKind::RParen, "Expected closing parenthesis")?;
                Ok(Factor::Paren(Box::new(expr)))
            }
            _ => {
                let token = self.current();
                Err(ParseError {
                    message: "Expected factor".to_string(),
                    line: token.line,
                    column: token.column,
                })
            }
        }
    }

    // Парсинг идентификатора (любое слово, не являющееся ключевым)
    // Identifier -> Word
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

    fn current_kind(&self) -> &LexemKind {
        &self.current().kind
    }

    fn is_eof(&self) -> bool {
        self.current().kind == LexemKind::Eof
    }
}
