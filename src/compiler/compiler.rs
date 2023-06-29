use std::collections::HashMap;

use num_traits::FromPrimitive;
use ordered_float::OrderedFloat;    

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
    pub prefix: Option<fn(&mut Compiler, Option<bool>)>,
    pub infix: Option<fn(&mut Compiler, Option<bool>)>,
    pub precedence: Precedence,
}

#[derive(Clone)]
pub struct Local {
    pub var: Token,
    pub depth: i32,
}

#[derive(Clone)]
pub struct Locals {
    pub list: Vec<Local>,
    pub scope_depth: i32,
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
    locals: Locals,
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
            locals: Locals {
                list: Vec::new(),
                scope_depth: 0,
            }
        };
        compi.init_rules();
        compi
    }

    pub fn compile(&mut self, chunk: &Chunk) -> bool {
        self.had_error = false;
        self.panic_mode = false;
        self.compiling_chunk = chunk.clone();

        //self.advance();

        while !self.match_next(TokenType::Eof) {
            self.declaration();
        }

        self.end_compiler();

        !self.had_error
    }

    fn declaration(&mut self) {
        if self.match_next(TokenType::Var) {
            self.var_declaration();
        } else if self.match_next(TokenType::LeftBrace) {
            self.begin_scope();
            self.block();
            self.end_scope();
        } else {
            self.statement();
        }

        if self.panic_mode {
            self.synchronize();
        }
    }

    fn synchronize(&mut self) {
        self.panic_mode = false;

        while self.current.token_type != TokenType::Eof {
            if self.previous.token_type == TokenType::Semicolon {
                return;
            }

            match self.current.token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }

    fn begin_scope(&mut self) {
        self.locals.scope_depth += 1;
    }

    fn end_scope(&mut self) {
        self.locals.scope_depth -= 1;

        while !self.locals.list.is_empty()
            && self.locals.list.last().unwrap().depth > self.locals.scope_depth
        {
            self.emit_byte(OpCode::Pop);
            self.locals.list.pop();
        }
    }

    fn block(&mut self) {
        while !self.check(TokenType::RightBrace) && !self.check(TokenType::Eof) {
            self.declaration();
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.");
    }

    fn statement(&mut self) {
        if self.match_next(TokenType::Print) {
            self.print_statement();
        } else {
            self.expression_statement();
        }
    }

    fn print_statement(&mut self) {
        self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.");
        self.emit_byte(OpCode::Print);
    }

    fn expression_statement(&mut self) {
        self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after expression.");
        //self.emit_byte(OpCode::Pop);
    }

    fn var_declaration(&mut self) {
        let global = self.parse_variable("Expect variable name.");

        if self.match_next(TokenType::Equal) {
            self.expression();
        } else {
            self.emit_byte(OpCode::Nil);
        }
        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.");

        self.define_variable(global);
    }

    fn define_variable(&mut self, name: String) {
        if self.locals.scope_depth > 0 {
            return;
        }

        let dg = OpCode::DefineGlobal(name);
        self.emit_byte(dg);
    }

    fn parse_variable(&mut self, error_message: &str) -> String {
        self.consume(TokenType::Identifier, error_message);

        self.declare_variable();
        if self.locals.scope_depth > 0 {
            return String::new();
        }

        self.previous.lexeme.clone()
    }

    fn declare_variable(&mut self) {
        if self.locals.scope_depth == 0 {
            return;
        }

        let local_list = self.locals.list.clone();
        for local in local_list.iter().rev() {
            if local.depth != -1 && local.depth < self.locals.scope_depth {
                break;
            }

            if self.previous.lexeme == local.var.lexeme {
                self.error("Already a variable with this name in this scope.");
            }
        }


        self.add_local(self.previous.clone());
    }

    fn add_local(&mut self, var: Token) {
        if self.locals.list.len() == u8::MAX as usize {
            self.error("Too many local variables in function.");
            return;
        }

        let local = Local {
            var,
            depth: -1,
        };
        self.locals.list.push(local);
    }

    fn match_next(&mut self, token_type: TokenType) -> bool {
        if !self.check(token_type) {
            return false;
        }
        self.advance();
        true
    }

    fn check(&self, token_type: TokenType) -> bool {
        self.current.token_type == token_type
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

    fn binary(&mut self, _can_assign: Option<bool>) {
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

    fn grouping(&mut self, _can_assign: Option<bool>) {
        self.expression();
        self.consume(
            TokenType::RightParen,
            "Expect ')' after expression.",
        );
    }

    fn number(&mut self, _can_assign: Option<bool>) {
        let value = self.previous.lexeme.parse::<f64>().unwrap();
        self.emit_constant(Value::Number(OrderedFloat(value)));
    }

    fn unary(&mut self, _can_assign: Option<bool>) {
        let operator_type = self.previous.token_type.clone();

        self.parse_precedence(Precedence::Unary);

        match operator_type {
            TokenType::Bang => self.emit_byte(OpCode::Not),
            TokenType::Minus => self.emit_byte(OpCode::Negate),
            _ => unreachable!(),
        }
    }
    fn literal(&mut self, _can_assign: Option<bool>) {
        let token_type = self.previous.token_type.clone();
        match token_type {
            TokenType::False => self.emit_byte(OpCode::False),
            TokenType::True => self.emit_byte(OpCode::True),
            TokenType::Nil => self.emit_byte(OpCode::Nil),
            _ => unreachable!(),
        }
    }

    fn variable(&mut self, can_assign: Option<bool>) {
        self.named_variable(can_assign.unwrap());
    }

    fn named_variable(&mut self, can_assign: bool) {

        let (get_op, set_op);
        let name = self.previous.lexeme.clone();
        let arg: isize = self.resolve_local(&name);
        if arg != -1 {
            get_op = OpCode::GetLocal(arg as usize);
            set_op = OpCode::SetLocal(arg as usize);
        } else {
            //let arg = self.identifier_constant(&self.current);
            get_op = OpCode::GetGlobal(name.clone());
            set_op = OpCode::SetGlobal(name);
        }

        //let name = self.previous.lexeme.clone();

        if can_assign & self.match_next(TokenType::Equal) {
            self.expression();
            self.emit_byte(set_op);
          } else {
            self.emit_byte(get_op);
          }
    }

    fn resolve_local(&mut self, name: &String) -> isize {
        for (i, local) in self.locals.list.iter().enumerate().rev() {
            if local.var.lexeme == name.to_string() {
                if local.depth == -1 {
                    self.error("Cannot read local variable in its own initializer.");
                }
                return i as isize;
            }
        }

        return -1;
    }

    fn string(&mut self, _can_assign: Option<bool>) { 
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

        let can_assign = precedence <= Precedence::Assignment;
        if let Some(prefix) = prefix_rule {
            prefix(self, Some(can_assign));
        }

        while precedence <= self.get_rule(&self.current.token_type).precedence {
            self.advance();
            let infix_rule = self.get_rule(&self.previous.token_type).infix;
            if infix_rule.is_none() {
                break;
            } else {
                infix_rule.unwrap()(self, Some(can_assign));
            }
        }

        if can_assign && self.match_next(TokenType::Equal) {
            self.error("Invalid assignment target.");
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

        self.rules.insert(TokenType::Identifier, 
            ParseRule {
                prefix: Some(Compiler::variable),
                infix: None,
                precedence: Precedence::None,
            },
        );
    }
}
