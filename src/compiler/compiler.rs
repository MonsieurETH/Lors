use std::collections::HashMap;

use super::{
    chunk::{Chunk, OpCode},
    scanner::{Scanner, Token, TokenType},
    value::Value,
};

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
    pub prefix: Option<fn()>,
    pub infix: Option<fn()>,
    pub precedence: Precedence,
}

pub struct Compiler {
    compiling_chunk: Chunk,
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
    debug_trace_execution: bool,
    rules: HashMap<TokenType, ParseRule>,
}

impl Compiler {
    pub fn new() -> Self {
        self.init_rules();
    }

    pub fn compile(self, source: String, chunk: Chunk) -> bool {
        let scanner = Scanner::init_scanner(source);
        self.had_error = false;
        self.panic_mode = false;
        self.compiling_chunk = chunk;

        self.advance();
        self.expression();
        self.consume(TokenType::Eof, "Expect end of expression.");
        self.end_compiler();

        !self.had_error
    }

    fn current_chunk(&self) -> &Chunk {
        &self.compiling_chunk
    }

    fn error_at_current(self, message: String) {
        self.error_at(self.current, message);
    }

    fn error(self, message: String) {
        self.error_at(self.previous, message);
    }

    fn error_at(self, token: Token, message: String) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        if token.token_type == TokenType::Eof {
            report(token.line, " at end", message);
        } else {
            report(token.line, format!(" at '{}'", token.lexeme), message);
        }

        self.had_error = true;
    }

    fn consume(self, token_type: TokenType, message: String) {
        if self.current.token_type == token_type {
            self.advance();
            return;
        }

        self.error_at_current(message);
    }

    fn emit_byte(self, byte: u8) {
        self.chunk.write_chunk(byte, self.previous.line);
    }

    fn emit_bytes(self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn end_compiler(self) {
        self.emit_return();
        if self.debug_trace_execution && !self.had_error {
            for chunk in self.current_chunk().code {
                println!("{:?}", chunk);
            }
        }
    }

    fn binary(self) {
        let operator_type = self.previous.token_type;

        let rule = self.get_rule(operator_type);
        self.parse_precedence(rule.precedence + 1);

        match operator_type {
            TokenType::Plus => self.emit_byte(OpCode::Add as u8),
            TokenType::Minus => self.emit_byte(OpCode::Subtract as u8),
            TokenType::Star => self.emit_byte(OpCode::Multiply as u8),
            TokenType::Slash => self.emit_byte(OpCode::Divide as u8),
            _ => unreachable!(),
        }
    }

    fn grouping(self) {
        self.expression();
        self.consume(
            TokenType::RightParen,
            "Expect ')' after expression.".to_string(),
        );
    }

    fn number(self) {
        let value = self.previous.lexeme.parse::<f64>().unwrap();
        self.emit_constant(Value::Number(value));
    }

    fn unary(self) {
        let operator_type = self.previous.token_type;

        self.parse_precedence(Precedence::Unary);

        match operator_type {
            TokenType::Minus => self.emit_byte(OpCode::Negate as u8),
            _ => unreachable!(),
        }
    }

    fn get_rule(self, token_type: TokenType) -> ParseRule {
        if self.rules.contains_key(&token_type) {
            return self.rules.get(&token_type).unwrap();
        }

        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        }
    }

    fn parse_precedence(self, precedence: Precedence) {
        self.advance();
        let prefix_rule = self.get_rule(self.previous.token_type).prefix;
        if prefix_rule.is_none() {
            self.error("Expect expression.".to_string());
            return;
        }

        prefix_rule.unwrap().call(self);

        while precedence <= self.get_rule(self.current.token_type).precedence {
            self.advance();
            let infix_rule = self.get_rule(self.previous.token_type).infix.unwrap();
            infix_rule.call(self);
        }
    }

    fn expression(self) {
        self.parse_precedence(Precedence::Assignment);
    }

    fn emit_return(self) {
        self.emit_byte(OpCode::Return as u8);
    }

    fn make_constant(self, value: f64) -> u8 {
        let constant = self.current_chunk().add_constant(value);
        if constant > u8::MAX {
            self.error("Too many constants in one chunk.".to_string());
            return 0;
        }

        constant as u8
    }

    fn emit_constant(self, value: f64) {
        self.emit_bytes(OpCode::Constant as u8, self.make_constant(value));
    }

    fn init_rules(self) {
        self.rules.insert(
            TokenType::LeftParen,
            ParseRule {
                prefix: Some(self.grouping),
                infix: None,
                precedence: Precedence::None,
            },
        );

        self.rules.insert(
            TokenType::Minus,
            ParseRule {
                prefix: Some(self.unary),
                infix: Some(self.binary),
                precedence: Precedence::None,
            },
        );

        self.rules.insert(
            TokenType::Plus,
            ParseRule {
                prefix: None,
                infix: Some(self.binary),
                precedence: Precedence::Term,
            },
        );

        self.rules.insert(
            TokenType::Slash,
            ParseRule {
                prefix: None,
                infix: Some(self.binary),
                precedence: Precedence::Factor,
            },
        );

        self.rules.insert(
            TokenType::Star,
            ParseRule {
                prefix: None,
                infix: Some(self.binary),
                precedence: Precedence::Factor,
            },
        );

        self.rules.insert(
            TokenType::Number,
            ParseRule {
                prefix: Some(self.number),
                infix: None,
                precedence: Precedence::None,
            },
        );

        self.rules.insert(
            TokenType::Eof,
            ParseRule {
                prefix: None,
                infix: None,
                precedence: Precedence::None,
            },
        );
    }
}
