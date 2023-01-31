use crate::value::Value;

#[derive(Debug)]
pub enum OpCode {
    OpReturn,
    OpConstant,
}
impl OpCode {
    pub fn disassemble(&self, chunk: &Chunk, offset: usize) {
        print!("{:04} ", offset);

        if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", chunk.lines[offset]);
        }

        match self {
            OpCode::OpConstant => {
                print!("{:>16?}", self);
                for val in &chunk.constants {
                    print!("{:?}", val);
                }
                println!();
            }
            _ => println!("{:?}", self),
        }
    }
}

pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    lines: Vec<u32>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: vec![],
            constants: vec![],
            lines: vec![],
        }
    }

    pub fn write(&mut self, op: OpCode, line: u32) {
        self.code.push(op);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn read_constant(&self, offset: usize) -> &Value {
        &self.constants[offset]
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);
        let mut offset: usize = 0;
        for op in &self.code {
            op.disassemble(self, offset);
            offset += 1;
        }
    }
}
