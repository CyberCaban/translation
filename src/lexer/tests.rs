use crate::lexer::*;

fn lex(input: &str) -> Vec<Lexem> {
    let mut lexer = Lexer::new();
    lexer.lex(input).expect("lexing failed")
}

#[test]
fn test_lex_with_spaces() {
    let input = "declare   Q   (   Name   )";
    let lexems = lex(input);
    assert_eq!(
        lexems.into_iter().map(|l| l.kind).collect::<Vec<_>>(),
        vec![
            LexemKind::Declare,
            LexemKind::Word("Q".to_string()),
            LexemKind::LParen,
            LexemKind::Word("Name".to_string()),
            LexemKind::RParen,
            LexemKind::Eof,
        ]
    );
}

#[test]
fn test_lex_conclusion_operator_tokens() {
    let input = "conclusion Q(x):-B(y)";
    let mut lexer = Lexer::new();
    let lexems = lexer.lex(input).expect("lexing failed");
    assert!(lexems.iter().any(|l| l.kind == LexemKind::Colon));
    assert!(lexems.iter().any(|l| l.kind == LexemKind::Minus));
}

#[test]
fn test_lex_error_unknown_char() {
    let input = "declare Q(Name) #";
    let mut lexer = Lexer::new();
    let error = lexer.lex(input).expect_err("expected lex error");
    assert!(error.message.contains("Unexpected character"));
}
