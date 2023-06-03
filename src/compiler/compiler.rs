pub struct Compiler {
    compiling_chunk: Chunk,
    current: Token,
    previous: Token,
    had_error: bool,
    panic_mode: bool,
}

impl Compiler {
    pub fn compile(&mut self, source: String, chunk: Chunk) -> bool {
        let scanner = Scanner::init_scanner(source);
        self.had_error = false;
        self.panice_mode = false;
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

    fn error_at_current(message: String) {
        error_at(&self.current, message);
    }

    fn error(message: String) {
        error_at(&self.previous, message);
    }

    fn error_at(token: Token, message: String) {
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

    fn consume(token_type: TokenType, message: String) {
        if self.current.token_type == token_type {
            self.advance();
            return;
        }

        self.error_at_current(message);
    }

    fn emit_byte(byte: u8) {
        self.chunk.write_chunk(byte, self.previous.line);
    }

    fn emit_bytes(byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn end_compiler() {
        self.emit_return();
        if self.debug_trace_execution && !self.had_error {
            self.current_chunk().disassemble("code");
        }
    }

    fn emit_return() {
        self.emit_byte(OpCode::Return as u8);
    }
}
