pub struct Compiler {
    chunk: Chunk,
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
        self.advance();
        self.expression();
        self.consume(TokenType::Eof, "Expect end of expression.");

        !self.had_error
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
}
