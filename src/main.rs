mod ast;
mod lexer;
mod operators;
mod parser;
pub mod tools;
mod visitors;

use ast::Error;
use lexer::Lexer;
use parser::Parser;
use std::{env, fs};

use crate::visitors::interpreter::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    match args.len() {
        2 => run_file(&args[1]),
        3 => run_test(&args[1]),
        _ => panic!("Usage: loxc [script]"),
    }
}

fn run_file(path: &String) {
    let content = fs::read_to_string(path).expect("Error reading file");
    let _had_error = run(&content);
}

fn run_test(path: &String) {
    let source = fs::read_to_string(path).expect("Error reading file");

    let mut lexer = Lexer::new(&source);
    lexer.scan_tokens();

    let mut parser: Parser = Parser::new(lexer.tokens);
    let ast = parser.parse();

    let mut visitor = Interpreter::new();
    for stmt in ast {
        let value = stmt.unwrap().accept(&mut visitor);
        match value {
            Err(Error { msg }) => println!("{:?}", msg),
            Ok(Some(v)) => println!("{:?}", v),
            _ => continue,
        }
    }
}

fn run(source: &String) -> bool {
    let mut lexer = Lexer::new(source);
    lexer.scan_tokens();

    let mut parser: Parser = Parser::new(lexer.tokens);
    let ast = parser.parse();

    let mut visitor = Interpreter::new();
    for stmt in ast {
        let value = stmt.unwrap().accept(&mut visitor);
        match value {
            Err(Error { msg }) => println!("{:?}", msg),
            Ok(Some(v)) => println!("{:?}", v),
            _ => continue,
        }
    }

    false
}

#[cfg(test)]
mod tests;
