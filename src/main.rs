use crate::interpreter::Interpreter;
use anyhow::{Context, Result};
use std::{env::args, fs::read_to_string, process};

mod interpreter;
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

    match run_program(&contents) {
        Ok(()) => {}
        Err(err) => {
            if let Some(parse_error) = err.downcast_ref::<crate::parser::ParseError>() {
                eprintln!(
                    "{}:{}:{}: {}",
                    filename, parse_error.line, parse_error.column, parse_error.message
                );
                process::exit(1);
            }

            if let Some(lex_error) = err.downcast_ref::<crate::lexer::LexError>() {
                eprintln!(
                    "{}:{}:{}: {}",
                    filename, lex_error.line, lex_error.column, lex_error.message
                );
                process::exit(1);
            }

            return Err(err);
        }
    }

    Ok(())
}

fn run_program(contents: &str) -> Result<()> {
    let interpreter = Interpreter::new(contents)?;
    interpreter.interpret()
}
