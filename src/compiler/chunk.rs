use super::value::Value;

#[derive(Debug, Clone)]
pub enum OpCode {
    Return,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Constant(Value),
    True,
    False,
    Nil,
    Not,
    Equal,
    Greater,
    Less,
    Print,
    Pop,
    DefineGlobal(String),
    GetGlobal(String),
    SetGlobal(String),
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write_chunk(&mut self, byte: OpCode, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value, line: usize) -> u8 {
        self.write_chunk(OpCode::Constant(value), line);

        (self.code.len() - 1) as u8
    }
}

pub struct ValueArray {
    pub values: Vec<u8>,
}
