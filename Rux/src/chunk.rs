use crate::value::Value;

#[derive(Debug)]
pub enum OpCode {
    Return,
    Constant(usize),
    Negate,

    Add,
    Subtract,
    Multiply,
    Divide,
}
impl OpCode {
    pub fn disassemble(&self, chunk: &Chunk, offset: usize) {
        print!("{:04} ", offset);

        if offset > 0 {
            print!("   | ");
        }
        // else {
        //     print!("{:4} ", chunk.lines[offset]);
        // }

        match self {
            OpCode::Constant(constant_offset) => {
                println!(
                    "Constant     {constant_offset} '{:?}'",
                    &chunk.constants[*constant_offset]
                )
            }
            op => println!("{:?}", op),
        }
    }
}

#[derive(Debug)]

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
    pub fn emit(&mut self, op: OpCode) {
        self.code.push(op);
    }

    pub fn emit_many(&mut self, ops: &mut Vec<OpCode>) {
        self.code.append(ops);
    }

    pub fn emit_constant(&mut self, val: Value) {
        let constant = self.add_constant(val);
        self.emit(OpCode::Constant(constant));
    }

    /// Get a reference to the chunk's code.
    pub fn code(&self) -> &[OpCode] {
        self.code.as_ref()
    }

    pub fn op_count(&self) -> usize {
        self.code.len()
    }

    pub fn op_get(&self, offset: usize) -> Option<&OpCode> {
        self.code.get(offset)
    }

    pub fn op_patch(&mut self, op_offset: usize, new_op: OpCode) {
        self.code[op_offset] = new_op;
    }
}
