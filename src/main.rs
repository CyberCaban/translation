use crate::{lexer::{LexemKind, Lexer}, parser::Parser};
use anyhow::{Context, Result};
use std::{env::args, fs::read_to_string};

mod lexer;
mod parser;

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        eprintln!("Filename was not provided!");
        return Ok(());
    }
    let filename = &args[1];
    let contents = read_to_string(filename).context(format!("File: {}", filename))?;

    let mut lexer = Lexer::new();
    let tokens = match lexer.lex(&contents) {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("Lexical error: {}", e);
            return Ok(());
        }
    };

    println!("Lexems:");
    for token in &tokens {
        if matches!(token.kind, LexemKind::Eof) {
            continue;
        }
        println!("{:?} at {}:{}", token.kind, token.line, token.column);
    }

    let mut parser = Parser::new(tokens);
    match parser.parse_program() {
        Ok(_) => println!("Syntax analysis: success"),
        Err(e) => eprintln!("Syntax error: {}", e),
    }

    Ok(())
}
