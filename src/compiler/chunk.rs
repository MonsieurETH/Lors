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
}

/*pub struct ChunkIterator<'a> {
        chunk: &'a Chunk,
        pos: usize,
}

impl<'a> Iterator for ChunkIterator<'a> {
    type Item = (&'a OpCode, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let code = self.chunk.code.get(self.pos);
        let line =  self.chunk.lines.get(self.pos);
        self.pos += 1;
        match (code, line) {
            (Some(code), Some(line)) => Some((code, *line)),
            _ => None
        }
    }
}*/

pub struct ValueArray {
    pub values: Vec<u8>,
}
