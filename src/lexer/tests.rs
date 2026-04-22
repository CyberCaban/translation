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
            LexemKind::Word("declare".to_string()),
            LexemKind::Word("Q".to_string()),
            LexemKind::LParen,
            LexemKind::Word("Name".to_string()),
            LexemKind::RParen,
            LexemKind::Eof,
        ]
    );
}

#[test]
fn test_lexem_kinds() {
    let input = [
        "=",
        "==",
        "=",
        "!=",
        "<",
        "<=",
        ">",
        ">=",
        "int",
        "float",
        "if",
        "then",
        "print",
        "begin",
        "end",
        "(",
        ")",
        ";",
        ",",
        ":",
        "-",
        "+",
        "*",
        "/",
        "1235",
        "3.14",
        "18446744073709551606", // very large 128bit number
    ]
    .join(" ");
    let mut lexer = Lexer::new();
    let lexems = lexer.lex(&input).expect("lexing failed");
    assert_eq!(
        lexems.into_iter().map(|l| l.kind).collect::<Vec<_>>(),
        vec![
            LexemKind::Assignment,
            LexemKind::Equal,
            LexemKind::Assignment,
            LexemKind::NotEqual,
            LexemKind::Less,
            LexemKind::LessEqual,
            LexemKind::Greater,
            LexemKind::GreaterEqual,
            LexemKind::Type(AType::Int),
            LexemKind::Type(AType::Float),
            LexemKind::If,
            LexemKind::Then,
            LexemKind::Print,
            LexemKind::Begin,
            LexemKind::End,
            LexemKind::LParen,
            LexemKind::RParen,
            LexemKind::Semicolon,
            LexemKind::Comma,
            LexemKind::Colon,
            LexemKind::Minus,
            LexemKind::Plus,
            LexemKind::Multiply,
            LexemKind::Divide,
            LexemKind::IntLiteral(1235),
            LexemKind::FloatLiteral(3.14),
            LexemKind::IntLiteral(18446744073709551606),
            LexemKind::Eof,
        ]
    );
}
