use crate::lexer::Lexer;
use crate::parser::Parser;

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
fn test_parse_valid_multiple_newline_without_semicolon() {
	let input = "declare Q(Aa)\ndeclare B(Bb)\nconclusion A(x):-Q(y),B(z)";
	let mut lexer = Lexer::new();
	let tokens = lexer.lex(input).expect("lexing failed");
	let mut parser = Parser::new(tokens);
	let program = parser.parse_program().expect("parsing failed");
	assert_eq!(program.declarations.len(), 3);
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
