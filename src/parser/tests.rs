use crate::lexer::Lexer;
use crate::parser::{Declaration, Parser, Value};

#[test]
fn test_parse_valid_program() {
	let input = "declare A(Alpha); conclusion Q(x,y,Id):-B(z),A(Name)";
	let mut lexer = Lexer::new();
	let tokens = lexer.lex(input).expect("lexing failed");
	let mut parser = Parser::new(tokens);
	let program = parser.parse_program().expect("parsing failed");
	assert_eq!(program.declarations.len(), 2);
}

#[test]
fn test_parse_valid_multiple_semicolon() {
	let input = "declare Q(Aa);declare B(Bb);conclusion A(x):-Q(y),B(z)";
	let mut lexer = Lexer::new();
	let tokens = lexer.lex(input).expect("lexing failed");
	let mut parser = Parser::new(tokens);
	let program = parser.parse_program().expect("parsing failed");
	assert_eq!(program.declarations.len(), 3);
}

#[test]
fn test_parse_error_multiple_newline_without_semicolon() {
	let input = "declare Q(Aa)\ndeclare B(Bb)\nconclusion A(x):-Q(y),B(z)";
	let mut lexer = Lexer::new();
	let tokens = lexer.lex(input).expect("lexing failed");
	let mut parser = Parser::new(tokens);
	let error = parser.parse_program().expect_err("expected parse error");
	assert!(error
		.message
		.contains("Unexpected token after end of program"));
}

#[test]
fn test_parse_error_missing_minus() {
	let input = "conclusion Q(x): B(y)";
	let mut lexer = Lexer::new();
	let tokens = lexer.lex(input).expect("lexing failed");
	let mut parser = Parser::new(tokens);
	let error = parser.parse_program().expect_err("expected parse error");
	assert!(error.message.contains("Expected '-' after ':'"));
}

#[test]
fn test_parse_error_missing_rparen() {
	let input = "declare Q(Name";
	let mut lexer = Lexer::new();
	let tokens = lexer.lex(input).expect("lexing failed");
	let mut parser = Parser::new(tokens);
	let error = parser.parse_program().expect_err("expected parse error");
	assert!(error.message.contains("Expected ')' after identifier"));
}

#[test]
fn test_parse_error_unknown_start() {
	let input = "hello Q(Name)";
	let mut lexer = Lexer::new();
	let tokens = lexer.lex(input).expect("lexing failed");
	let mut parser = Parser::new(tokens);
	let error = parser.parse_program().expect_err("expected parse error");
	assert!(error.message.contains("Expected 'declare' or 'conclusion'"));
}

#[test]
fn test_parse_valid_declare_ast_shape() {
	let input = "declare Q(IdentifierOnly)";
	let mut lexer = Lexer::new();
	let tokens = lexer.lex(input).expect("lexing failed");
	let mut parser = Parser::new(tokens);
	let program = parser.parse_program().expect("parsing failed");

	assert_eq!(program.declarations.len(), 1);
	match &program.declarations[0] {
		Declaration::Declare { func, identifier } => {
			assert_eq!(func, "Q");
			assert_eq!(identifier, "IdentifierOnly");
		}
		_ => panic!("expected declare declaration"),
	}
}

#[test]
fn test_parse_valid_conclusion_with_three_calls() {
	let input = "conclusion A(x,y,Name):-Q(z),B(Id),A(AlphaBeta)";
	let mut lexer = Lexer::new();
	let tokens = lexer.lex(input).expect("lexing failed");
	let mut parser = Parser::new(tokens);
	let program = parser.parse_program().expect("parsing failed");

	assert_eq!(program.declarations.len(), 1);
	match &program.declarations[0] {
		Declaration::Conclusion { left, right } => {
			assert_eq!(left.func, "A");
			assert_eq!(left.args.len(), 3);
			assert_eq!(left.args[0], Value::Variable('x'));
			assert_eq!(left.args[1], Value::Variable('y'));
			assert_eq!(left.args[2], Value::Identifier("Name".to_string()));

			assert_eq!(right.len(), 3);
			assert_eq!(right[0].func, "Q");
			assert_eq!(right[1].func, "B");
			assert_eq!(right[2].func, "A");
		}
		_ => panic!("expected conclusion declaration"),
	}
}

#[test]
fn test_parse_valid_single_letter_identifier_not_variable() {
	let input = "conclusion Q(a):-B(b),A(c)";
	let mut lexer = Lexer::new();
	let tokens = lexer.lex(input).expect("lexing failed");
	let mut parser = Parser::new(tokens);
	let program = parser.parse_program().expect("parsing failed");

	match &program.declarations[0] {
		Declaration::Conclusion { left, right } => {
			assert_eq!(left.args[0], Value::Identifier("a".to_string()));
			assert_eq!(right[0].args[0], Value::Identifier("b".to_string()));
			assert_eq!(right[1].args[0], Value::Identifier("c".to_string()));
		}
		_ => panic!("expected conclusion declaration"),
	}
}

#[test]
fn test_parse_error_unknown_function_in_declare() {
	let input = "declare C(Name)";
	let mut lexer = Lexer::new();
	let tokens = lexer.lex(input).expect("lexing failed");
	let mut parser = Parser::new(tokens);
	let error = parser.parse_program().expect_err("expected parse error");
	assert!(error.message.contains("Expected function name: Q, B or A"));
}

#[test]
fn test_parse_error_unknown_function_in_conclusion_right_side() {
	let input = "conclusion Q(x):-C(y)";
	let mut lexer = Lexer::new();
	let tokens = lexer.lex(input).expect("lexing failed");
	let mut parser = Parser::new(tokens);
	let error = parser.parse_program().expect_err("expected parse error");
	assert!(error.message.contains("Expected function name: Q, B or A"));
}

#[test]
fn test_parse_error_missing_call_after_minus() {
	let input = "conclusion Q(x):-";
	let mut lexer = Lexer::new();
	let tokens = lexer.lex(input).expect("lexing failed");
	let mut parser = Parser::new(tokens);
	let error = parser.parse_program().expect_err("expected parse error");
	assert!(error.message.contains("Expected function name: Q, B or A"));
}

#[test]
fn test_parse_error_trailing_comma_in_arguments() {
	let input = "conclusion Q(x,):-B(y)";
	let mut lexer = Lexer::new();
	let tokens = lexer.lex(input).expect("lexing failed");
	let mut parser = Parser::new(tokens);
	let error = parser.parse_program().expect_err("expected parse error");
	assert!(error.message.contains("Expected identifier"));
}

#[test]
fn test_parse_error_trailing_comma_in_right_calls() {
	let input = "conclusion Q(x):-B(y),";
	let mut lexer = Lexer::new();
	let tokens = lexer.lex(input).expect("lexing failed");
	let mut parser = Parser::new(tokens);
	let error = parser.parse_program().expect_err("expected parse error");
	assert!(error.message.contains("Expected function name: Q, B or A"));
}

#[test]
fn test_parse_error_trailing_semicolon() {
	let input = "declare Q(Name);";
	let mut lexer = Lexer::new();
	let tokens = lexer.lex(input).expect("lexing failed");
	let mut parser = Parser::new(tokens);
	let error = parser.parse_program().expect_err("expected parse error");
	assert!(error.message.contains("Expected 'declare' or 'conclusion'"));
}

#[test]
fn test_parse_error_unexpected_tokens_after_program() {
	let input = "declare Q(Name) declare B(Other)";
	let mut lexer = Lexer::new();
	let tokens = lexer.lex(input).expect("lexing failed");
	let mut parser = Parser::new(tokens);
	let error = parser.parse_program().expect_err("expected parse error");
	assert!(error.message.contains("Unexpected token after end of program"));
}

#[test]
fn test_parse_error_empty_input() {
	let input = "";
	let mut lexer = Lexer::new();
	let tokens = lexer.lex(input).expect("lexing failed");
	let mut parser = Parser::new(tokens);
	let error = parser.parse_program().expect_err("expected parse error");
	assert!(error.message.contains("Expected 'declare' or 'conclusion'"));
}
