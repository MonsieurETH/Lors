mod ast;
mod lexer;
mod operators;
mod parser;
pub mod tools;
mod visitors;

use ast::{Error, Expr, IVisitorExpr, IVisitorStmt, Stmt};
use lexer::Lexer;
use parser::Parser;
use std::{env, fs};
use visitors::{interpreter::Interpreter, resolver::Resolver};

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

    let mut interpreter: Interpreter = Interpreter::new();

    let only_ok = ast.iter().filter(|result| result.is_ok());
    if only_ok.count() == ast.len() {
        let mut resolver: Resolver = Resolver::new(&mut interpreter);
        apply_visitor(&mut resolver, &ast);
    } else {
        let only_err = ast.iter().filter(|result| result.is_err());
        for err in only_err {
            println!("{:?}", err.as_ref().unwrap_err());
        }
    }

    apply_visitor(&mut interpreter, &ast);
    let only_err = ast.iter().filter(|result| result.is_err());
}

type ResultStmt = Result<Option<Stmt>, Error>;
type ResultExpr = Result<Option<Expr>, Error>;

fn apply_visitor<T>(visitor: &mut T, ast: &Vec<Result<Stmt, Error>>)
where
    T: IVisitorExpr<ResultExpr> + IVisitorStmt<ResultStmt>,
{
    for stmt in ast {
        let value = stmt.as_ref().unwrap().accept(visitor);
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

    let mut interpreter: Interpreter = Interpreter::new();

    let only_ok = ast.iter().filter(|result| result.is_ok());
    if only_ok.count() == ast.len() {
        let mut resolver: Resolver = Resolver::new(&mut interpreter);
        apply_visitor(&mut resolver, &ast);
    } else {
        let only_err = ast.iter().filter(|result| result.is_err());
        for err in only_err {
            println!("{:?}", err.as_ref().unwrap_err());
        }
    }

    apply_visitor(&mut interpreter, &ast);
    let only_err = ast.iter().filter(|result| result.is_err());

    false
}

#[cfg(test)]
mod tests;
