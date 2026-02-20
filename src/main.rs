use crate::lexer::Lexer;
use anyhow::{Context, Result};
use std::{
    env::{Args, args},
    fs::read_to_string,
};

mod lexer;

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        eprintln!("Filename was not provided!");
        return Ok(());
    }
    let filename = &args[1];
    let contents = read_to_string(&filename).context(format!("File: {}", filename))?;
    let mut lexer = Lexer::new();
    lexer.lex(&contents);
    println!("Found lexems:```");
    lexer.print_lexems();
    println!("```");
    Ok(())
}
