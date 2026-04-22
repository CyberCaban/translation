use crate::lexer::Lexer;

use super::*;

fn parse_program(input: &str) -> Program {
    let mut lexer = Lexer::new();
    let tokens = lexer.lex(input).expect("lexing failed");
    let mut parser = Parser::new(tokens);
    parser.parse_program().expect("parsing failed")
}

#[test]
fn parses_program_with_single_declaration_and_print() {
    let program = parse_program("int x begin print x end");

    assert_eq!(
        program,
        Program {
            vars_decls: vec![VarsDecl::Var {
                ttype: AType::Int,
                name: "x".to_string(),
            }],
            body: vec![Statement::Print {
                value: Expr::Term(Term::Factor(Factor::Variable("x".to_string()))),
            }],
        }
    );
}

#[test]
fn parses_program_with_multiple_declarations() {
    let program = parse_program("int x; float y begin print x end");

    assert_eq!(
        program.vars_decls,
        vec![
            VarsDecl::Var {
                ttype: AType::Int,
                name: "x".to_string(),
            },
            VarsDecl::Var {
                ttype: AType::Float,
                name: "y".to_string(),
            },
        ]
    );
}

#[test]
fn parses_assignment_with_operator_precedence() {
    let program = parse_program("int x begin x = 1 + 2 * 3 end");

    assert_eq!(
        program.body,
        vec![Statement::Assignment {
            var: "x".to_string(),
            value: Expr::Plus {
                left: Box::new(Expr::Term(Term::Factor(Factor::IntLiteral(1)))),
                term: Term::Multiply {
                    left: Box::new(Term::Factor(Factor::IntLiteral(2))),
                    factor: Factor::IntLiteral(3),
                },
            },
        }]
    );
}

#[test]
fn parses_parenthesized_expression_in_print() {
    let program = parse_program("int x begin print (1 + 2) end");

    assert_eq!(
        program.body,
        vec![Statement::Print {
            value: Expr::Term(Term::Factor(Factor::Paren(Box::new(Expr::Plus {
                left: Box::new(Expr::Term(Term::Factor(Factor::IntLiteral(1)))),
                term: Term::Factor(Factor::IntLiteral(2)),
            })))),
        }]
    );
}

#[test]
fn parses_if_with_comparison_and_nested_body() {
    let program = parse_program("int x begin if x < 10 then begin print x end end");

    assert_eq!(
        program.body,
        vec![Statement::If {
            condition: Comparison::Less(
                Expr::Term(Term::Factor(Factor::Variable("x".to_string()))),
                Expr::Term(Term::Factor(Factor::IntLiteral(10))),
            ),
            then_branch: vec![Statement::Print {
                value: Expr::Term(Term::Factor(Factor::Variable("x".to_string()))),
            }],
        }]
    );
}

#[test]
fn parses_float_literal_in_print() {
    let program = parse_program("float y begin print 3.14 end");

    assert_eq!(
        program.body,
        vec![Statement::Print {
            value: Expr::Term(Term::Factor(Factor::FloatLiteral(3.14))),
        }]
    );
}

#[test]
fn rejects_program_without_begin() {
    let mut lexer = Lexer::new();
    let tokens = lexer.lex("int x print x end").expect("lexing failed");
    let mut parser = Parser::new(tokens);

    let error = parser.parse_program().expect_err("expected parse error");
    assert_eq!(error.message, "No begin keyword");
}

#[test]
fn rejects_statement_without_expression() {
    let mut lexer = Lexer::new();
    let tokens = lexer.lex("int x begin print end").expect("lexing failed");
    let mut parser = Parser::new(tokens);

    let error = parser.parse_program().expect_err("expected parse error");
    assert_eq!(error.message, "Expected factor");
}
