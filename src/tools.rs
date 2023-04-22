use glob::glob;
use std::{collections::HashMap, fs};

use crate::{ast::Stmt, lexer::Lexer, parser::Parser, visitors::interpreter::Interpreter};

const TESTS_FOLDER: &str = "test";

pub struct TestReader {
    test_source: HashMap<String, String>,
}

impl TestReader {
    pub fn new() -> Self {
        let paths =
            glob(&(TESTS_FOLDER.to_string() + "/*/*.lox")).expect("Failed to read glob pattern");

        let mut sources: HashMap<String, String> = HashMap::new();

        for entry in paths {
            match entry {
                Ok(mut path) => {
                    let source = fs::read_to_string(path.clone()).expect("Failed to read file");

                    let filename = path.file_name().unwrap().to_str().unwrap();
                    let file_removed = path.parent().unwrap();
                    let folder = file_removed.file_name().unwrap().to_str().unwrap();
                    let test_name = folder.to_string() + "/" + filename;
                    //println!("{}", test_name);
                    sources.insert(test_name, source);
                }
                Err(e) => println!("ERROR {:?}", e),
            }
        }

        TestReader {
            test_source: sources,
        }
    }

    pub fn run_test(&self, test_path: &str) -> Vec<Stmt> {
        let source = self.get_test_source(test_path);
        let results = self.run_source(&source);
        let expected = self.get_expected_result(&test_path);
        println!("expected: {:?}", expected);
        println!("results: {:?}", results);

        results
    }

    fn get_test_source(&self, test_path: &str) -> String {
        self.test_source[test_path].clone()
    }

    fn get_expected_result(&self, test_path: &str) -> Vec<Result<String, String>> {
        let source = self.get_test_source(test_path);

        let mut comments = Vec::new();
        let lines = source.lines();
        for line in lines {
            let comment = line.trim().split("//").nth(1);

            if let Some(comment) = comment {
                if comment.trim().starts_with("expect:") {
                    let splitted = comment.split(" ").collect::<Vec<&str>>();
                    let expected = splitted.last().unwrap().to_string();
                    comments.push(Ok(expected));
                } else {
                    comments.push(Err(comment.trim().to_string()));
                }
            }
        }

        comments
    }

    fn run_source(&self, source: &str) -> Vec<Stmt> {
        let mut lexer = Lexer::new(source);
        lexer.scan_tokens();

        let mut parser: Parser = Parser::new(lexer.tokens);
        let ast = parser.parse();

        let mut visitor = Interpreter::new();
        let mut results = vec![];
        for stmt in ast {
            let value = stmt.accept(&mut visitor);
            match value {
                None => continue,
                Some(value) => {
                    if let Stmt::Expression(expr) = stmt {}
                    results.push(value);
                }
            }
        }

        results
    }
}
