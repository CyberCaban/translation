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

#[test]
fn parses_program_with_single_declaration_and_print() {
    let program = parse_program("int x begin print x end");

    assert_eq!(program.vars_decls.len(), 1);
    assert_eq!(program.body.len(), 1);

    match &program.vars_decls[0] {
        VarsDecl::Var {
            ttype,
            name,
            line,
            column,
        } => {
            assert_eq!(ttype, &AType::Int);
            assert_eq!(name, "x");
            assert_eq!((*line, *column), (1, 1));
        }
    }

    match &program.body[0] {
        Statement::Print { value, line, column } => {
            assert_eq!((*line, *column), (1, 13));
            assert!(matches!(
                value,
                Expr::Term(Term::Factor(Factor::Variable { name, line, column }))
                if name == "x" && *line == 1 && *column == 19
            ));
        }
        other => panic!("unexpected statement: {:?}", other),
    }
}

#[test]
fn parses_program_with_multiple_declarations() {
    let program = parse_program("int x; float y begin print x end");

    assert_eq!(program.vars_decls.len(), 2);

    assert!(matches!(
        &program.vars_decls[0],
        VarsDecl::Var {
            ttype: AType::Int,
            name,
            line: 1,
            column: 1
        } if name == "x"
    ));

    assert!(matches!(
        &program.vars_decls[1],
        VarsDecl::Var {
            ttype: AType::Float,
            name,
            line: 1,
            column: 8
        } if name == "y"
    ));
}

#[test]
fn parses_assignment_with_operator_precedence() {
    let program = parse_program("int x begin x = 1 + 2 * 3 end");

    match &program.body[0] {
        Statement::Assignment {
            var,
            value,
            line,
            column,
        } => {
            assert_eq!(var, "x");
            assert_eq!((*line, *column), (1, 13));
            assert!(matches!(
                value,
                Expr::Plus { left, term, line: 1, column: 19 }
                if matches!(left.as_ref(), Expr::Term(Term::Factor(Factor::IntLiteral { value: 1, line: 1, column: 17 })))
                    && matches!(term, Term::Multiply { left, factor, line: 1, column: 23 }
                        if matches!(left.as_ref(), Term::Factor(Factor::IntLiteral { value: 2, line: 1, column: 21 }))
                            && matches!(factor, Factor::IntLiteral { value: 3, line: 1, column: 25 }))
            ));
        }
        other => panic!("unexpected statement: {:?}", other),
    }
}

#[test]
fn parses_parenthesized_expression_in_print() {
    let program = parse_program("int x begin print (1 + 2) end");

    match &program.body[0] {
        Statement::Print { value, .. } => {
            assert!(matches!(
                value,
                Expr::Term(Term::Factor(Factor::Paren { expr, line: 1, column: 19 }))
                if matches!(expr.as_ref(), Expr::Plus { left, term, line: 1, column: 22 }
                    if matches!(left.as_ref(), Expr::Term(Term::Factor(Factor::IntLiteral { value: 1, line: 1, column: 20 })))
                        && matches!(term, Term::Factor(Factor::IntLiteral { value: 2, line: 1, column: 24 })))
            ));
        }
        other => panic!("unexpected statement: {:?}", other),
    }
}

#[test]
fn parses_if_with_comparison_and_nested_body() {
    let program = parse_program("int x begin if x < 10 then begin print x end end");

    match &program.body[0] {
        Statement::If {
            condition,
            then_branch,
            line,
            column,
        } => {
            assert_eq!((*line, *column), (1, 13));
            assert_eq!(then_branch.len(), 1);
            assert!(matches!(
                condition,
                Comparison::Less { left, right, line: 1, column: 18 }
                if matches!(left, Expr::Term(Term::Factor(Factor::Variable { name: _, line: 1, column: 16 })))
                    && matches!(right, Expr::Term(Term::Factor(Factor::IntLiteral { value: 10, line: 1, column: 20 })))
            ));
        }
        other => panic!("unexpected statement: {:?}", other),
    }
}

#[test]
fn parses_float_literal_in_print() {
    let program = parse_program("float y begin print 3.14 end");

    match &program.body[0] {
        Statement::Print { value, .. } => {
            assert!(matches!(
                value,
                Expr::Term(Term::Factor(Factor::FloatLiteral { value: v, line: 1, column: 21 }))
                if (*v - 3.14).abs() < f64::EPSILON
            ));
        }
        other => panic!("unexpected statement: {:?}", other),
    }
}

#[test]
fn rejects_program_without_begin() {
    let mut lexer = Lexer::new();
    let tokens = lexer.lex("int x print x end").expect("lexing failed");
    let mut parser = Parser::new(tokens);

    let error = parser.parse_program().expect_err("expected parse error");
    assert_eq!(error.message, "missing `begin`");
    assert_eq!((error.line, error.column), (1, 7));
}

#[test]
fn rejects_statement_without_expression() {
    let mut lexer = Lexer::new();
    let tokens = lexer.lex("int x begin print end").expect("lexing failed");
    let mut parser = Parser::new(tokens);

    let error = parser.parse_program().expect_err("expected parse error");
    assert_eq!(error.message, "expected factor");
    assert!(error.line > 0 && error.column > 0);
}

#[test]
fn rejects_assignment_with_type_mismatch() {
    let error = parse_and_verify_program("int x begin x = 3.14 end")
        .expect_err("expected type error");

    assert!(error.message.contains("type mismatch in assignment"));
    assert_eq!((error.line, error.column), (1, 17));
}

#[test]
fn rejects_arithmetic_with_type_mismatch() {
    let error = parse_and_verify_program("int x begin x = 1 + 2.5 end")
        .expect_err("expected type error");

    assert!(error.message.contains("type mismatch in arithmetic operation"));
    assert_eq!((error.line, error.column), (1, 19));
}