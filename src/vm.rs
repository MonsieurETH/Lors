pub struct VM {
    pub chunk: Chunk,
    pub ip: usize
}

pub enum InterpretResult {
    INTERPRET_OK,
    INTERPRET_COMPILE_ERROR,
    INTERPRET_RUNTIME_ERROR
}

impl VM {
    pub fn initVM() -> VM {
        VM { chunk: Chunk::new(), ip: 0 }
    }

    pub fn interpret(&mut self, chunk: Chunk) {
        self.chunk = chunk;
        self.ip = 0;

        return self.run();
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {
            let instruction = self.read_byte();
            match instruction {
                OpCode::OP_RETURN => {
                    return InterpretResult::INTERPRET_OK;
                }
                OpCode::OP_CONSTANT => {
                    let constant = self.read_constant();
                    print!("{}", constant);
                }
            }

            self.ip.inc();
        }
    }

    fn read_byte(&mut self) -> OpCode {
        let byte = self.chunk.code[self.ip];
        self.ip.inc();
        byte
    }

    fn read_constant(&mut self) -> Value {
        let index = self.read_byte();
        self.chunk.constants.values[constant as usize]
    }
}