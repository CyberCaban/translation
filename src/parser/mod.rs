#[cfg(test)]
mod tests;

mod error;
mod program;

pub use error::ParseError;
pub use program::{Comparison, Expr, Factor, Program, Statement, Term, VarsDecl};

use crate::{
    lexer::{Lexem, LexemKind},
    parser::program::Var,
};

pub struct Parser {
    tokens: Vec<Lexem>,
    idx: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Lexem>) -> Self {
        Self { tokens, idx: 0 }
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut declarations = Vec::new();

        // VarsDecls
        while self.current_kind() != &LexemKind::Begin {
            declarations.push(self.parse_declaration()?);
            self.expect_kind(&LexemKind::Semicolon, "missing semicolon")?;
            if self.current_kind() == &LexemKind::Begin {
                break;
            }
        }

        // Body
        let body = self.parse_body()?;

        if !self.is_eof() {
            let token = self.current();
            return Err(ParseError::new(
                "unexpected token after end of program",
                token.line,
                token.column,
            ));
        }

        Ok(Program {
            vars_decls: declarations,
            body,
        })
    }

    fn parse_declaration(&mut self) -> Result<VarsDecl, ParseError> {
        let token = self.current().clone();
        if let LexemKind::Type(atype) = token.kind {
            self.idx += 1;
            let mut idents = vec![Var::new(
                atype.clone(),
                self.parse_identifier()?,
                token.line,
                token.column,
            )];
            loop {
                if self.match_kind(&LexemKind::Comma) {
                    idents.push(Var::new(
                        atype.clone(),
                        self.parse_identifier()?,
                        token.line,
                        token.column,
                    ));
                } else {
                    break;
                }
            }
            return Ok(VarsDecl::new(idents));
        }

        Err(ParseError::new(
            "expected variable declaration",
            token.line,
            token.column,
        ))
    }

    fn parse_body(&mut self) -> Result<Vec<Statement>, ParseError> {
        self.expect_kind(&LexemKind::Begin, "missing `begin`")?;
        let mut statements = Vec::new();

        while !self.is_eof() {
            statements.push(self.parse_statement()?);
            self.expect_kind(&LexemKind::Semicolon, "missing semicolon")?;
            if self.current().kind == LexemKind::End {
                break;
            }
        }

        self.expect_kind(&LexemKind::End, "missing `end`")?;
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        let token = self.current().clone();
        match token.kind {
            LexemKind::Word(_) => self.parse_assignment(token.line, token.column),
            LexemKind::If => self.parse_if(token.line, token.column),
            LexemKind::Print => self.parse_print(token.line, token.column),
            _ => Err(ParseError::new(
                "expected statement",
                token.line,
                token.column,
            )),
        }
    }

    fn parse_assignment(&mut self, line: usize, column: usize) -> Result<Statement, ParseError> {
        let var = self.parse_identifier()?;
        self.expect_kind(&LexemKind::Assignment, "expected assignment operator")?;
        let value = self.parse_expr()?;
        Ok(Statement::Assignment {
            var,
            value,
            line,
            column,
        })
    }

    fn parse_if(&mut self, line: usize, column: usize) -> Result<Statement, ParseError> {
        self.expect_kind(&LexemKind::If, "expected `if`")?;
        let condition = self.parse_comparison()?;
        self.expect_kind(&LexemKind::Then, "expected `then`")?;
        let then_branch = self.parse_body()?;
        Ok(Statement::If {
            condition,
            then_branch,
            line,
            column,
        })
    }

    fn parse_print(&mut self, line: usize, column: usize) -> Result<Statement, ParseError> {
        self.expect_kind(&LexemKind::Print, "expected `print`")?;
        let value = self.parse_expr()?;
        Ok(Statement::Print {
            value,
            line,
            column,
        })
    }

    fn parse_comparison(&mut self) -> Result<Comparison, ParseError> {
        let left = self.parse_expr()?;
        let token = self.current().clone();

        match token.kind {
            LexemKind::Equal => {
                self.idx += 1;
                Ok(Comparison::Equal {
                    left,
                    right: self.parse_expr()?,
                    line: token.line,
                    column: token.column,
                })
            }
            LexemKind::NotEqual => {
                self.idx += 1;
                Ok(Comparison::NotEqual {
                    left,
                    right: self.parse_expr()?,
                    line: token.line,
                    column: token.column,
                })
            }
            LexemKind::Less => {
                self.idx += 1;
                Ok(Comparison::Less {
                    left,
                    right: self.parse_expr()?,
                    line: token.line,
                    column: token.column,
                })
            }
            LexemKind::LessEqual => {
                self.idx += 1;
                Ok(Comparison::LessEqual {
                    left,
                    right: self.parse_expr()?,
                    line: token.line,
                    column: token.column,
                })
            }
            LexemKind::Greater => {
                self.idx += 1;
                Ok(Comparison::Greater {
                    left,
                    right: self.parse_expr()?,
                    line: token.line,
                    column: token.column,
                })
            }
            LexemKind::GreaterEqual => {
                self.idx += 1;
                Ok(Comparison::GreaterEqual {
                    left,
                    right: self.parse_expr()?,
                    line: token.line,
                    column: token.column,
                })
            }
            _ => Ok(Comparison::Expression(left)),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let mut expr = Expr::Term(self.parse_term()?);

        while let LexemKind::Plus | LexemKind::Minus = self.current_kind() {
            let token = self.current().clone();
            self.idx += 1;
            let term = self.parse_term()?;
            expr = match token.kind {
                LexemKind::Plus => Expr::Plus {
                    left: Box::new(expr),
                    term,
                    line: token.line,
                    column: token.column,
                },
                LexemKind::Minus => Expr::Minus {
                    left: Box::new(expr),
                    term,
                    line: token.line,
                    column: token.column,
                },
                _ => unreachable!(),
            };
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Term, ParseError> {
        let mut term = Term::Factor(self.parse_factor()?);

        while let LexemKind::Multiply | LexemKind::Divide = self.current_kind() {
            let token = self.current().clone();
            self.idx += 1;
            let factor = self.parse_factor()?;
            term = match token.kind {
                LexemKind::Multiply => Term::Multiply {
                    left: Box::new(term),
                    factor,
                    line: token.line,
                    column: token.column,
                },
                LexemKind::Divide => Term::Divide {
                    left: Box::new(term),
                    factor,
                    line: token.line,
                    column: token.column,
                },
                _ => unreachable!(),
            };
        }

        Ok(term)
    }

    fn parse_factor(&mut self) -> Result<Factor, ParseError> {
        let token = self.current().clone();

        match token.kind {
            LexemKind::Word(_) => {
                let name = self.parse_identifier()?;
                Ok(Factor::Variable {
                    name,
                    line: token.line,
                    column: token.column,
                })
            }
            LexemKind::IntLiteral(value) => {
                self.idx += 1;
                Ok(Factor::IntLiteral {
                    value,
                    line: token.line,
                    column: token.column,
                })
            }
            LexemKind::FloatLiteral(value) => {
                self.idx += 1;
                Ok(Factor::FloatLiteral {
                    value,
                    line: token.line,
                    column: token.column,
                })
            }
            LexemKind::LParen => {
                self.idx += 1;
                let expr = self.parse_expr()?;
                self.expect_kind(&LexemKind::RParen, "expected closing parenthesis")?;
                Ok(Factor::Paren {
                    expr: Box::new(expr),
                    line: token.line,
                    column: token.column,
                })
            }
            LexemKind::Type(ref atype) => {
                self.idx += 1;
                self.expect_kind(
                    &LexemKind::LParen,
                    "expected opening parenthesis after cast type",
                )?;
                let expr = self.parse_expr()?;
                self.expect_kind(&LexemKind::RParen, "expected closing parenthesis")?;
                Ok(Factor::Cast {
                    target: atype.clone(),
                    expr: Box::new(expr),
                    line: token.line,
                    column: token.column,
                })
            }
            _ => Err(ParseError::new("expected factor", token.line, token.column)),
        }
    }

    fn parse_identifier(&mut self) -> Result<String, ParseError> {
        let token = self.current().clone();
        match token.kind {
            LexemKind::Word(word) => {
                self.idx += 1;
                Ok(word)
            }
            _ => Err(ParseError::new(
                "expected identifier",
                token.line,
                token.column,
            )),
        }
    }

    fn expect_kind(&mut self, expected: &LexemKind, message: &str) -> Result<(), ParseError> {
        if self.match_kind(expected) {
            Ok(())
        } else {
            let token = self.current();
            Err(ParseError::new(message, token.line, token.column))
        }
    }

    fn match_kind(&mut self, expected: &LexemKind) -> bool {
        if self.current().kind == *expected {
            self.idx += 1;
            true
        } else {
            false
        }
    }

    fn current(&self) -> &Lexem {
        &self.tokens[self.idx.min(self.tokens.len().saturating_sub(1))]
    }

    fn current_kind(&self) -> &LexemKind {
        &self.current().kind
    }

    fn is_eof(&self) -> bool {
        self.current().kind == LexemKind::Eof
    }
}
