use glob::glob;
use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

use crate::interpreter::ast::{Expr, Literal};

pub struct TestReader {
    test_source: HashMap<String, String>,
}

impl TestReader {
    pub fn new(pattern: &str) -> Self {
        let paths = glob(pattern).expect("Failed to read glob pattern");

        let mut sources: HashMap<String, String> = HashMap::new();

        for entry in paths {
            match entry {
                Ok(path) => {
                    let source = fs::read_to_string(path.clone()).expect("Failed to read file");

                    let filename = path.file_name().unwrap().to_str().unwrap();
                    let file_removed = path.parent().unwrap();
                    let folder = file_removed.file_name().unwrap().to_str().unwrap();
                    let test_name = folder.to_string() + "/" + filename;
                    sources.insert(test_name, source);
                }
                Err(e) => println!("ERROR {:?}", e),
            }
        }

        TestReader {
            test_source: sources,
        }
    }

    pub fn run_test(&self, test_path: &str) -> (Vec<String>, Vec<String>) {
        let results = self.run_source(test_path);
        let expected = self
            .get_expected_result(&test_path)
            .iter()
            .map(|x| match x {
                Ok(v) => format!("{:?}", v.clone()),
                Err(e) => format!("{:?}", e.trim().clone()),
            })
            .collect::<Vec<String>>();
        println!("expected: {:?}", expected);
        println!("results: {:?}", results);

        (expected, results)
    }

    fn get_test_source(&self, test_path: &str) -> String {
        let path = test_path
            .split('/')
            .into_iter()
            .skip(2)
            .collect::<Vec<&str>>()
            .join("/");
        self.test_source[path.as_str()].clone()
    }

    fn get_expected_result(&self, test_path: &str) -> Vec<Result<Expr, String>> {
        let source = self.get_test_source(test_path);

        let mut comments = Vec::new();
        let lines = source.lines();
        for line in lines {
            let comment = line.trim().split("//").nth(1);

            if let Some(comment) = comment {
                if comment.trim().starts_with("expect:") {
                    let splitted = comment.split(" ").collect::<Vec<&str>>();
                    let expected = splitted.last().unwrap().to_string();
                    if expected.chars().nth(0).unwrap().is_numeric() {
                        comments.push(Ok(Expr::Literal(Literal::Number(
                            expected.parse().unwrap(),
                        ))));
                    } else if expected.chars().nth(0).unwrap() == '-' {
                        comments.push(Ok(Expr::Literal(Literal::Number(
                            expected.parse().unwrap(),
                        ))));
                    } else if expected.as_str() == "true" {
                        comments.push(Ok(Expr::Literal(Literal::Bool(true))));
                    } else if expected.as_str() == "false" {
                        comments.push(Ok(Expr::Literal(Literal::Bool(false))));
                    } else if expected.as_str() == "nil" {
                        comments.push(Ok(Expr::Literal(Literal::Nil)));
                    } else {
                        comments.push(Ok(Expr::Literal(Literal::Str(expected))));
                    }
                } else if comment.trim().starts_with("expect runtime error: ") {
                    let splitted = comment.split(":").collect::<Vec<&str>>();
                    let expected = splitted.last().unwrap().to_string();
                    comments.push(Err(expected));
                } else {
                    comments.push(Err(comment.trim().to_string()));
                }
            }
        }

        comments
    }

    fn run_source(&self, source: &str) -> Vec<String> {
        let mut cmd = Command::new("cargo")
            .arg("run") // ensure unbuffered
            .arg(source)
            .arg("--test")
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let stdout = cmd.stdout.take().unwrap();

        let mut bufread = BufReader::new(stdout);
        let mut buf = String::new();

        let mut results = vec![];
        while let Ok(n) = bufread.read_line(&mut buf) {
            if n > 0 {
                results.push(buf.trim().clone().to_string());
                buf.clear();
            } else {
                break;
            }
        }
        results
    }

    pub fn iter(&self) -> Vec<&String> {
        self.test_source.keys().collect()
    }
}
