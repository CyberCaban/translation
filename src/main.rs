use crate::lexer::Lexer;
use anyhow::{Context, Result};
use std::fs::read_to_string;

mod lexer;

fn main() -> Result<()> {
    let filename = "./urls.tx";
    let contents = read_to_string(filename).context(format!("File: {}", filename))?;
    let mut lexer = Lexer::new();
    lexer.lex(&contents);
    println!("Found lexems:```");
    lexer.print_lexems(&contents);
    println!("```");
    Ok(())
}
