mod interpreter;
pub mod tools;

use interpreter::ast::{Error, Expr, IVisitorExpr, IVisitorStmt, Stmt};
use interpreter::lexer::Lexer;
use interpreter::parser::Parser;
use interpreter::visitors::{interpreter::Interpreter, resolver::Resolver};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
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
    let res = lexer.scan_tokens();
    match res {
        Ok(_) => {}
        Err(e) => {
            println!("{:?}", e.msg);
            return;
        }
    }

    let mut parser: Parser = Parser::new(lexer.tokens);
    let ast = parser.parse();

    let mut interpreter: Interpreter = Interpreter::new();

    let only_ok = ast.iter().filter(|result| result.is_ok());
    if only_ok.count() != ast.len() {
        let only_err = ast.iter().filter(|result| result.is_err());
        for err in only_err {
            println!("{:?}", err.as_ref().unwrap_err().msg);
            return;
        }
    }
    let mut resolver: Resolver = Resolver::new(&mut interpreter);
    let res = apply_visitor(&mut resolver, &ast);

    if !res {
        return;
    }

    apply_visitor(&mut interpreter, &ast);

    let _ = ast.iter().filter(|result| result.is_err());
}

type ResultStmt = Result<Option<Stmt>, Error>;
type ResultExpr = Result<Option<Expr>, Error>;

fn apply_visitor<T>(visitor: &mut T, ast: &Vec<Result<Stmt, Error>>) -> bool
where
    T: IVisitorExpr<ResultExpr> + IVisitorStmt<ResultStmt>,
{
    let mut clean = true;
    for stmt in ast {
        let value = stmt.as_ref().unwrap().accept(visitor);
        match value {
            Err(Error { msg }) => {
                println!("{:?}", msg);
                clean = false;
            }
            Ok(Some(v)) => println!("{:?}", v),
            _ => continue,
        }
    }
    clean
}

fn run(source: &String) -> bool {
    let mut lexer = Lexer::new(&source);
    let res = lexer.scan_tokens();
    match res {
        Ok(_) => {}
        Err(e) => {
            println!("{:?}", e.msg);
            return false;
        }
    }
    let mut parser: Parser = Parser::new(lexer.tokens);
    let ast = parser.parse();

    let mut interpreter: Interpreter = Interpreter::new();

    let only_ok = ast.iter().filter(|result| result.is_ok());
    if only_ok.count() != ast.len() {
        let only_err = ast.iter().filter(|result| result.is_err());
        for err in only_err {
            println!("{:?}", err.as_ref().unwrap_err().msg);
            return true;
        }
    }
    let mut resolver: Resolver = Resolver::new(&mut interpreter);
    let res = apply_visitor(&mut resolver, &ast);

    if !res {
        return true;
    }

    apply_visitor(&mut interpreter, &ast);
    let _ = ast.iter().filter(|result| result.is_err());

    false
}

#[cfg(test)]
mod tests;
