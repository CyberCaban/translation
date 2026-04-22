use anyhow::Result;

use crate::{lexer::Lexer, parser::Parser};

pub struct Interpreter {
    lexer: Lexer,
    parser: Parser,
}

impl Interpreter {
    pub fn new(input: &str) -> Result<Interpreter> {
        let mut lexer = Lexer::new();
        let tokens = lexer.lex(input)?;
        let mut parser = Parser::new(tokens);
        parser.parse_program()?;
        Ok(Self { lexer, parser })
    }
}
