use crate::interpreter::Interpreter;
use anyhow::{Context, Result};
use std::{env::args, fs::read_to_string};

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

    let interpreter = Interpreter::new(&contents)?;

    Ok(())
}
