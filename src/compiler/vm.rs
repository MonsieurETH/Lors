pub struct VM {
    pub chunk: Chunk,
    pub ip: usize,
    pub stack: Stack,
    pub debug_trace_execution: bool,
}

pub struct Stack {
    pub values: Vec<Value>,
}

impl Stack {
    pub fn push(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn pop(&mut self) -> Value {
        self.values.pop().unwrap()
    }
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl VM {
    pub fn initVM() -> VM {
        let stack = Stack { values: Vec::new() };
        VM {
            chunk: Chunk::new(),
            ip: 0,
            stack,
            debug_trace_execution: false,
        }
    }

    pub fn interpret(&mut self, source: String) {
        if !self.compile(source) {
            return InterpretResult::CompileError;
        }
        self.chunk = chunk;
        self.ip = 0;

        self.run()
    }

    pub fn reset_stack(&mut self) {
        self.stack.values.clear();
    }

    fn runtime_error(&mut self, message: String) {
        println!("{}", message);
        self.reset_stack();
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {
            let instruction = self.read_byte();
            match instruction {
                OpCode::Return => {
                    println!("{}", self.stack.pop());
                    return InterpretResult::Ok;
                }
                OpCode::Negate => {
                    let value = self.stack.pop();
                    match value {
                        Some(Value::Number(n)) => self.stack.push(Value::Number(-n)),
                        None => return InterpretResult::RuntimeError,
                    }
                    print!("{}", value);
                }
                OpCode::Add => self.binary_op(Value::Number, u32::add),
                OpCode::Subtract => self.binary_op(Value::Number, u32::sub),
                OpCode::Multiply => self.binary_op(Value::Number, u32::mul),
                OpCode::Divide => self.binary_op(Value::Number, u32::div),
                OpCode::Constant => {
                    let constant = self.read_constant();
                    self.stack.push(constant);
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
        self.chunk.constants.values[index as usize]
    }

    fn binary_op(&mut self, value_type: Value, op: fn(u32, u32) -> u32) {
        let top = self.stack.values.len();
        if top < 2 {
            return self.runtime_error("Stack underflow".to_string());
        }
        let b = self.stack.pop();
        let a = self.stack.pop();
        if !a.is_number() || !b.is_number() {
            return self.runtime_error("Operands must be numbers".to_string());
        }
        self.stack.push(op(a, b));
    }
}
