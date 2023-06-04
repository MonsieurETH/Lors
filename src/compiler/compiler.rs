use std::collections::HashMap;

use super::{
    chunk::{Chunk, OpCode},
    scanner::{Scanner, Token, TokenType},
    value::Value,
};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

pub struct ParseRule {
    pub prefix: Option<fn(&mut Compiler)>,
    pub infix: Option<fn(&mut Compiler)>,
    pub precedence: Precedence,
}

pub struct Compiler {
    compiling_chunk: Chunk,
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
    debug_trace_execution: bool,
    scanner: Scanner,
    rules: HashMap<TokenType, ParseRule>,
}

impl Compiler {
    pub fn new() -> Self {
        let mut compi = Self {
            compiling_chunk: Chunk::new(),
            current: Token::new(),
            previous: Token::new(),
            had_error: false,
            panic_mode: false,
            debug_trace_execution: false,
            scanner: Scanner::init_scanner(String::new()),
            rules: HashMap::new(),
        };
        compi.init_rules();
        compi
    }

    pub fn compile(&mut self, source: String, chunk: Chunk) -> bool {
        let mut scanner = Scanner::init_scanner(source);
        self.had_error = false;
        self.panic_mode = false;
        self.compiling_chunk = chunk;

        self.advance();
        self.expression();
        self.consume(TokenType::Eof, String::from("Expect end of expression."));

        self.end_compiler();

        !self.had_error
    }

    fn advance(&mut self) {
        self.previous = self.current;

        loop {
            self.current = self.scanner.scan_token();
            if self.current.token_type != TokenType::Error {
                break;
            }

            self.error_at_current(self.current.lexeme);
        }
    }

    fn current_chunk(&self) -> &Chunk {
        &self.compiling_chunk
    }

    fn error_at_current(&mut self, message: String) {
        self.error_at(self.current, message);
    }

    fn error(&mut self, message: String) {
        self.error_at(self.previous, message);
    }

    fn error_at(&mut self, token: Token, message: String) {
        if self.panic_mode {
            return;
        }
        if token.token_type == TokenType::Eof {
            println!("Error at end: {}", message);
        } else {
            println!("Error at line {}: {}", token.line, message)
        }

        self.had_error = true;
    }

    fn consume(&mut self, token_type: TokenType, message: String) {
        if self.current.token_type == token_type {
            self.advance();
            return;
        }

        self.error_at_current(message);
    }

    fn emit_byte(&mut self, byte: OpCode) {
        self.compiling_chunk.write_chunk(byte, self.previous.line);
    }

    fn emit_bytes(&mut self, byte1: OpCode, byte2: OpCode) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn end_compiler(&mut self) {
        self.emit_return();
        if self.debug_trace_execution && !self.had_error {
            for chunk in self.current_chunk().code {
                println!("{:?}", chunk);
            }
        }
    }

    fn binary(&mut self) {
        let operator_type = self.previous.token_type;

        let rule = self.get_rule(operator_type);
        self.parse_precedence(rule.precedence + 1);

        match operator_type {
            TokenType::Plus => self.emit_byte(OpCode::Add ),
            TokenType::Minus => self.emit_byte(OpCode::Subtract ),
            TokenType::Star => self.emit_byte(OpCode::Multiply ),
            TokenType::Slash => self.emit_byte(OpCode::Divide ),
            _ => unreachable!(),
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(
            TokenType::RightParen,
            "Expect ')' after expression.".to_string(),
        );
    }

    fn number(&mut self) {
        let value = self.previous.lexeme.parse::<f64>().unwrap();
        self.emit_constant(Value::Number(value));
    }

    fn unary(&mut self) {
        let operator_type = self.previous.token_type;

        self.parse_precedence(Precedence::Unary);

        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::Negate ),
            _ => unreachable!(),
        }
    }

    fn get_rule(self, token_type: TokenType) -> ParseRule {
        if self.rules.contains_key(&token_type) {
            return *self.rules.get(&token_type).unwrap();
        }

        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let prefix_rule = self.get_rule(self.previous.token_type).prefix;
        if prefix_rule.is_none() {
            self.error("Expect expression.".to_string());
            return;
        }

        if let Some(prefix) = prefix_rule {
            prefix(self);
        }
        //prefix_rule.unwrap().call(self);

        while precedence <= self.get_rule(self.current.token_type).precedence {
            self.advance();
            let infix_rule = self.get_rule(self.previous.token_type).infix;
            if infix_rule.is_none() {
                break;
            } else {
                infix_rule.unwrap()(self);
            }
        }
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn emit_return(&mut self) {
        self.emit_byte(OpCode::Return );
    }

    fn emit_constant(&mut self, value: Value) {
        self.current_chunk().add_constant(value, 0);
        //self.emit_bytes(OpCode::Constant, self.make_constant(value));
    }

    fn init_rules(&mut self) {
        self.rules.insert(
            TokenType::LeftParen,
            ParseRule {
                prefix: Some(Compiler::grouping),
                infix: None,
                precedence: Precedence::None,
            },
        );

        self.rules.insert(
            TokenType::Minus,
            ParseRule {
                prefix: Some(Compiler::unary),
                infix: Some(Compiler::binary),
                precedence: Precedence::None,
            },
        );

        self.rules.insert(
            TokenType::Plus,
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Term,
            },
        );

        self.rules.insert(
            TokenType::Slash,
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Factor,
            },
        );

        self.rules.insert(
            TokenType::Star,
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Factor,
            },
        );

        self.rules.insert(
            TokenType::Number,
            ParseRule {
                prefix: Some(Compiler::number),
                infix: None,
                precedence: Precedence::None,
            },
        );
    }
}
