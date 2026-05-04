use crate::lexer::{AType, Lexer};

use super::*;

fn parse_program(input: &str) -> Program {
    let mut lexer = Lexer::new();
    let tokens = lexer.lex(input).expect("lexing failed");
    let mut parser = Parser::new(tokens);
    parser.parse_program().expect("parsing failed")
}

fn parse_and_verify_program(input: &str) -> Result<Program, ParseError> {
    let mut lexer = Lexer::new();
    let tokens = lexer.lex(input).expect("lexing failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("parsing failed");
    program.verify_invariants().map(|_| program)
}
