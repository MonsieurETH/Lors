mod ast;
mod lexer;
mod operators;
mod parser;
mod tools;
mod visitors;

use lexer::Lexer;
use parser::Parser;
use std::{env, fs};
use tools::TestReader;

use crate::visitors::interpreter::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    match args.len() {
        2 => run_file(&args[1]),
        _ => panic!("Usage: loxc [script]"),
    }
}

fn run_file(path: &String) {
    let content = fs::read_to_string(path).expect("Error reading file");
    let _had_error = run(&content);
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
        let value = stmt.accept(&mut visitor).unwrap();
        match value {
            Some(v) => println!("{:?}", v),
            None => continue,
        }
    }

    //println!("----------------");

    //let tr = TestReader::new();
    //let res = tr.run_test("./test/assignment/syntax.lox");

    //println!("ACA {:?}", res);

    //println!("{:?}", visitor.env);

    false
}

#[cfg(test)]
mod tests {
    use std::result;

    use crate::tools::TestReader;

    #[test]
    fn it_works() {
        let tr = TestReader::new();
        let (expected, result) = tr.run_test("./test/assignment/syntax.lox");
        assert_eq!(expected, result)
    }
}
