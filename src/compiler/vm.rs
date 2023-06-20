use super::{chunk::{Chunk, OpCode}, value::Value, compiler::Compiler};

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

    pub fn pop(&mut self) -> Option<Value> {
        self.values.pop()
    }
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

impl VM {
    pub fn init_vm() -> VM {
        let stack = Stack { values: Vec::new() };
        VM {
            chunk: Chunk::new(),
            ip: 0,
            stack,
            debug_trace_execution: false,
        }
    }

    pub fn interpret(&mut self, source: &String) -> InterpretResult{

        let mut compi = Compiler::new(source);

        if !compi.compile(&self.chunk) {
            return InterpretResult::CompileError;
        } else {
            self.chunk = compi.compiling_chunk.clone();
        }
        self.ip = 0;
        println!("Code: {:?}", self.chunk.code);

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
                    println!("{:?}", self.stack.pop());
                    return InterpretResult::Ok;
                }
                OpCode::Negate => {
                    let value = self.stack.pop();
                    match value {
                        Some(Value::Number(n)) => self.stack.push(Value::Number(-n)),
                        _ => return InterpretResult::RuntimeError,
                    }
                    print!("{:?}", value.unwrap());
                }
                OpCode::Add => self.binary_op(OpCode::Add),
                OpCode::Subtract => self.binary_op(OpCode::Subtract),
                OpCode::Multiply => self.binary_op(OpCode::Multiply),
                OpCode::Divide => self.binary_op(OpCode::Divide),
                OpCode::Constant(value) => {
                    //let constant: Value = self.read_constant();
                    print!("{:?}", value);
                    self.stack.push(value);
                }
            }

            //self.ip += 1;
        }
    }

    fn read_byte(&mut self) -> OpCode {
        let byte = &self.chunk.code[self.ip];
        self.ip += 1;
        byte.clone()
    }

    fn read_constant(&mut self) -> Value {
        let index = self.read_byte();
        match index {
            OpCode::Constant(value) => value,
            _ => unreachable!(),
        }
        //self.chunk.constants[index as usize].clone()
    }

    fn binary_op(&mut self, op: OpCode) {
        let top = self.stack.values.len();
        println!("top: {}", top);
        if top < 2 {
            return self.runtime_error("Stack underflow".to_string());
        }
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        if !a.is_number() || !b.is_number() {
            return self.runtime_error("Operands must be numbers".to_string());
        }

        let res = match op {
            OpCode::Add => a.as_number() + b.as_number(),
            OpCode::Subtract => a.as_number() - b.as_number(),
            OpCode::Multiply => a.as_number() * b.as_number(),
            OpCode::Divide => a.as_number() / b.as_number(),
            _ => unreachable!(),
        };

        self.stack.push(Value::from_f64(res));
    }
}
