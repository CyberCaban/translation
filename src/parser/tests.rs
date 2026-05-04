use crate::lexer::Lexer;

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

#[test]
fn parses_if_then_single_statement() {
    let program = parse_program(
        "int x; begin x = 1; if x == 1 then begin print x; end; end",
    );

    assert_eq!(program.body.len(), 2);

    match &program.body[1] {
        Statement::If { then_branch, .. } => {
            assert_eq!(then_branch.len(), 1);
            assert!(matches!(then_branch[0], Statement::Print { .. }));
        }
        _ => panic!("expected if statement"),
    }
}

#[test]
fn parses_if_then_block() {
    let program = parse_program(
        "int x; begin x = 1; if x == 1 then begin print x; end; end",
    );

    assert_eq!(program.body.len(), 2);

    match &program.body[1] {
        Statement::If { then_branch, .. } => {
            assert_eq!(then_branch.len(), 1);
            assert!(matches!(then_branch[0], Statement::Print { .. }));
        }
        _ => panic!("expected if statement"),
    }
}

#[test]
fn verifies_if_condition_and_branch_types() {
    let result = parse_and_verify_program(
        "int x, y; begin x = 10; if x <= 10 then begin y = 1; end; end",
    );

    assert!(result.is_ok());
}
