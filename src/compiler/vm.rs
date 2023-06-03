pub struct VM {
    pub chunk: Chunk,
    pub ip: usize,
    pub stack: Stack,
    pub debug_trace_execution: bool,
}

type Value = f64;

pub struct Stack {
    pub values: Vec<Value>,
    pub stack_top: usize,
}

impl Stack {
    pub fn push(&mut self, value: Value) {
        self.values.push(value);
        self.stack_top += 1;
    }

    pub fn pop(&mut self) -> Value {
        self.stack_top -= 1;
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
        let stack = Stack {
            values: Vec::new(),
            stack_top: 0,
        };
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

        return self.run();
    }

    pub fn reset_stack(&mut self) {
        self.stack.stack_top = 0;
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {
            let instruction = self.read_byte();
            match instruction {
                OpCode::Return => {
                    println!("{}", self.stack.pop());
                    return InterpretResult::Ok;
                }
                OpCode::NEGATE => {
                    self.stack.push(-self.stack.pop());
                    print!("{}", value);
                }
                OpCode::Add => self.binary_op(u32::add),
                OpCode::Subtract => self.binary_op(u32::sub),
                OpCode::Multiply => self.binary_op(u32::mul),
                OpCode::Divide => self.binary_op(u32::div),
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

    fn binary_op(&mut self, op: fn(u32, u32) -> u32) {
        let b = self.stack.pop();
        let a = self.stack.pop();
        self.stack.push(op(a, b));
    }
}
