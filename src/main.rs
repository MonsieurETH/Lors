mod ast;
mod lexer;
mod parser;
mod visitors;

use lexer::Lexer;
use parser::Parser;
use std::{env, fs};
use visitors::interpreter::Interpreter;
//use visitors::printer::AstPrinter;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    match args.len() {
        //0 => run_promt(),
        2 => run_file(&args[1]),
        _ => panic!("Usage: loxc [script]"),
    }
}

//fn run_promt() { }

fn run_file(path: &String) {
    let content = fs::read_to_string(path).expect("Error reading file");
    let had_error = run(&content);
}

fn run(source: &String) -> bool {
    let mut lexer = Lexer::new(source);
    lexer.scan_tokens();

    //for token in lexer.tokens.into_iter() {
    //    println!("{:?}", token);
    //}

    let mut parser: Parser = Parser::new(lexer.tokens);
    let ast = parser.parse();

    //println!("{:?}", ast);
    //let mut visitor = AstPrinter {};
    //let ast: String = ast.accept(&mut visitor);

    //println!("{:?}", ast);

    let mut visitor = Interpreter::new();
    for stmt in ast {
        let value = stmt.accept(&mut visitor);
        println!("{:?}", value);
    }

    println!("{:?}", visitor.env);

    false
}
