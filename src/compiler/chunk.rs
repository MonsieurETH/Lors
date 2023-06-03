#[derive(Debug, Clone)]
pub enum OpCode {
    Return,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Constant,
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

    pub fn add_constant(&mut self, value: u8, line: usize) {
        self.write_chunk(OpCode::Constant, line);
        self.write_chunk(value, line);
    }
}

pub struct ValueArray {
    pub values: Vec<u8>,
}
