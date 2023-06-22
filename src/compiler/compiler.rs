use std::collections::HashMap;

use num_traits::FromPrimitive;    

use super::{
    chunk::{Chunk, OpCode},
    scanner::{Scanner, Token, TokenType},
    value::Value,
};

#[derive(Debug, PartialEq, PartialOrd, Clone, FromPrimitive)]
pub enum Precedence {
    None = 0,
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

impl Precedence {
    fn next(self) -> Precedence {
        match FromPrimitive::from_u8(self as u8 + 1) {
            Some(d2) => d2,
            None => FromPrimitive::from_u8(0).unwrap(),
        }
    }
}

#[derive(Clone)]
pub struct ParseRule {
    pub prefix: Option<fn(&mut Compiler)>,
    pub infix: Option<fn(&mut Compiler)>,
    pub precedence: Precedence,
}



pub struct Compiler {
    pub compiling_chunk: Chunk,
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
    debug_trace_execution: bool,
    scanner: Scanner,
    rules: HashMap<TokenType, ParseRule>,
}

impl Compiler {
    pub fn new(source: &String) -> Self {
        let mut scanner = Scanner::init_scanner(source);
        let current = scanner.scan_token();
        let mut compi = Self {
            compiling_chunk: Chunk::new(),
            current,
            previous: Token::new(),
            had_error: false,
            panic_mode: false,
            debug_trace_execution: false,
            scanner,
            rules: HashMap::new(),
        };
        compi.init_rules();
        compi
    }

    pub fn compile(&mut self, chunk: &Chunk) -> bool {
        self.had_error = false;
        self.panic_mode = false;
        self.compiling_chunk = chunk.clone();

        self.advance();
        self.expression();
        //self.consume(TokenType::Eof, "Expect end of expression.");

        self.end_compiler();

        !self.had_error
    }

    fn advance(&mut self) {
        self.previous = self.current.clone();

        loop {
            self.current = self.scanner.scan_token();
            if self.current.token_type != TokenType::Error {
                break;
            }

            let lexeme = self.current.lexeme.clone();
            self.error_at_current(&lexeme);
        }
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.compiling_chunk
    }

    fn error_at_current(&mut self, message: &str) {
        let curr = self.current.clone();
        self.error_at(&curr, message);
    }

    fn error(&mut self, message: &str) {
        let prev = self.previous.clone();
        self.error_at(&prev, message);
    }

    fn error_at(&mut self, token: &Token, message: &str) {
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

    fn consume(&mut self, token_type: TokenType, message: &str) {
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
            for chunk in &self.current_chunk().code {
                println!("{:?}", chunk);
            }
        }
    }

    fn binary(&mut self) {
        let operator_type = self.previous.token_type.clone();

        let rule = self.get_rule(&operator_type);
        self.parse_precedence(rule.precedence.next());

        match operator_type {
            TokenType::Plus => self.emit_byte(OpCode::Add),
            TokenType::Minus => self.emit_byte(OpCode::Subtract),
            TokenType::Star => self.emit_byte(OpCode::Multiply),
            TokenType::Slash => self.emit_byte(OpCode::Divide),
            TokenType::BangEqual => self.emit_bytes(OpCode::Equal, OpCode::Not),
            TokenType::EqualEqual => self.emit_byte(OpCode::Equal),
            TokenType::Greater => self.emit_byte(OpCode::Greater),
            TokenType::GreaterEqual => self.emit_bytes(OpCode::Less, OpCode::Not),
            TokenType::Less => self.emit_byte(OpCode::Less),
            TokenType::LessEqual => self.emit_bytes(OpCode::Greater, OpCode::Not),
            _ => unreachable!(),
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(
            TokenType::RightParen,
            "Expect ')' after expression.",
        );
    }

    fn number(&mut self) {
        let value = self.previous.lexeme.parse::<f64>().unwrap();
        self.emit_constant(Value::Number(value));
    }

    fn unary(&mut self) {
        let operator_type = self.previous.token_type.clone();

        self.parse_precedence(Precedence::Unary);

        match operator_type {
            TokenType::Bang => self.emit_byte(OpCode::Not),
            TokenType::Minus => self.emit_byte(OpCode::Negate),
            _ => unreachable!(),
        }
    }
    fn literal(&mut self) {
        let token_type = self.previous.token_type.clone();
        match token_type {
            TokenType::False => self.emit_byte(OpCode::False),
            TokenType::True => self.emit_byte(OpCode::True),
            TokenType::Nil => self.emit_byte(OpCode::Nil),
            _ => unreachable!(),
        }
    }

    fn string(&mut self) { 
        let value = self.previous.lexeme.clone();
        self.emit_constant(Value::String(value));
    }

    fn get_rule(&self, token_type: &TokenType) -> ParseRule {
        if self.rules.contains_key(&token_type) {
            return self.rules.get(&token_type).unwrap().clone();
        }

        ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::None,
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence) {
        self.advance();
        let prefix_rule = self.get_rule(&self.previous.token_type).prefix;
        if prefix_rule.is_none() {
            self.error(&"Expect expression.");
            return;
        }

        if let Some(prefix) = prefix_rule {
            prefix(self);
        }

        while precedence <= self.get_rule(&self.current.token_type).precedence {
            self.advance();
            let infix_rule = self.get_rule(&self.previous.token_type).infix;
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
        let chunk = self.current_chunk();
        chunk.add_constant(value, 0);
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
                precedence: Precedence::Term,
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

        self.rules.insert(
            TokenType::False, 
            ParseRule {
                prefix: Some(Compiler::literal),
                infix: None,
                precedence: Precedence::None,
            },
        );

        self.rules.insert(
            TokenType::True, 
            ParseRule {
                prefix: Some(Compiler::literal),
                infix: None,
                precedence: Precedence::None,
            },
        );

        self.rules.insert(
            TokenType::Nil, 
            ParseRule {
                prefix: Some(Compiler::literal),
                infix: None,
                precedence: Precedence::None,
            },
        );

        self.rules.insert(TokenType::Bang, 
            ParseRule {
                prefix: Some(Compiler::unary),
                infix: None,
                precedence: Precedence::None,
            },
        );

        self.rules.insert(TokenType::BangEqual, 
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Equality,
            },
        );

        self.rules.insert(TokenType::EqualEqual, 
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Equality,
            },
        );

        self.rules.insert(TokenType::Greater, 
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Comparison,
            },
        );

        self.rules.insert(TokenType::GreaterEqual, 
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Comparison,
            },
        );

        self.rules.insert(TokenType::Less, 
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Comparison,
            },
        );

        self.rules.insert(TokenType::LessEqual, 
            ParseRule {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: Precedence::Comparison,
            },
        );

        self.rules.insert(TokenType::String, 
            ParseRule {
                prefix: Some(Compiler::string),
                infix: None,
                precedence: Precedence::None,
            },
        );
    }
}
