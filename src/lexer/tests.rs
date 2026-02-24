use core::panic;

use crate::lexer::*;

fn process(input: &str) -> Vec<Lexem> {
    let mut lexer = Lexer::new();
    lexer.lex(input);
    lexer.parsed_lexems
}

#[test]
fn test_simple() {
    let input = "https://example.com";
    let lexems = process(input);
    match &lexems[0] {
        Lexem::Url(url) => assert_eq!(url, input),
        _ => panic!("Expected url lexem"),
    }
}

#[test]
fn test_empty() {
    let input = "";
    let lexems = process(input);
    assert_eq!(lexems.first(), None::<&Lexem>);
}

#[test]
fn test_url_long() {
    let input = "https://example.com/files/download.zip";
    let lexems = process(input);
    match &lexems[0] {
        Lexem::Url(url) => assert_eq!(url, input),
        _ => panic!("Expected URL lexem"),
    }
}

#[test]
fn test_url_whitespaces() {
    let input = "\t \n\t\nhttps://example.com\t\n\t\n \t";
    let lexems = process(input);
    match &lexems[0] {
        Lexem::Url(url) => assert_eq!(url, "https://example.com"),
        _ => panic!("Expected URL lexem"),
    }
}

#[test]
fn test_multiple_urls() {
    let input = " fdosn   fef   s    https://site1.com   
        an     d http://site2.org a   n  dhtpp http:/ http://www.site3.net";
    let lexems = process(input);

    assert_eq!(lexems.len(), 3);

    match &lexems[0] {
        Lexem::Url(url) => assert_eq!(url, "https://site1.com"),
        _ => panic!("Expected URL lexem"),
    }

    match &lexems[1] {
        Lexem::Url(url) => assert_eq!(url, "http://site2.org"),
        _ => panic!("Expected URL lexem"),
    }

    match &lexems[2] {
        Lexem::Url(url) => assert_eq!(url, "http://www.site3.net"),
        _ => panic!("Expected URL lexem"),
    }
}
