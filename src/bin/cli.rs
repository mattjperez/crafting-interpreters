use core::panic;
use logos::Logos;
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3.. => {
            println!("usage: jlox [script]");
            std::process::exit(64);
        }
        2 => run_file(&args[1])?,
        _ => run_prompt(),
    }

    Ok(())
}

use crafting_interpreters::token::Token;

fn run_file(fname: &str) -> Result<(), Box<dyn Error>> {
    // convert to path
    let source = std::fs::read_to_string(fname)?;
    let lex = Token::lexer(&source);
    for result in lex {
        match result {
            Ok(token) => println!("{:#?}", token),
            Err(e) => panic!("error while lexing: {:?}", e),
        }
    }
    Ok(())
}

fn run_prompt() {
    todo!();
}
