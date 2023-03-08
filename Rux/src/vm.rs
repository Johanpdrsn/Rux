use crate::chunk::{Chunk, OpCode};
use crate::stack::Stack;
use crate::value::Value;

#[derive(Debug)]
pub struct VM {
    pub chunk: Chunk,
    pub ip: usize,
    pub stack: Stack,
}

pub type InterpretResult<T> = Result<T, RuntimeError>;

#[derive(Debug)]
pub enum RuntimeError {
    NoMoreOperations(usize),
    Other(String),
}

impl RuntimeError {
    pub fn new(message: &str) -> Self {
        Self::Other(message.to_string())
    }
}

impl VM {
    pub fn new() -> Self {
        VM {
            chunk: Chunk::new(),
            ip: 0,
            stack: Stack::new(),
        }
    }

    pub fn run(&mut self) -> InterpretResult<()> {
        loop {
            let op = self.chunk.op_get(self.ip);

            for val in self.stack.contents() {
                println!("[ {val} ]");
            }
            op.unwrap().disassemble(&self.chunk, self.ip);

            self.ip += 1;

            match op {
                Some(OpCode::Return) => {
                    println!("{}", self.stack.pop().unwrap());
                    return Ok(());
                }
                Some(OpCode::Constant(iid)) => {
                    let constant = self.chunk.read_constant(*iid);
                    self.stack.push(constant.clone());
                }
                Some(OpCode::Negate) => {
                    let n = self.stack.pop_number()?;
                    let res = -n;
                    self.stack.push(Value::Number(res));
                }
                Some(OpCode::Add) => VM::binary(&mut self.stack, |a, b| Value::Number(a + b))?,
                Some(OpCode::Subtract) => VM::binary(&mut self.stack, |a, b| Value::Number(a - b))?,
                Some(OpCode::Multiply) => VM::binary(&mut self.stack, |a, b| Value::Number(a * b))?,
                Some(OpCode::Divide) => VM::binary(&mut self.stack, |a, b| Value::Number(a / b))?,
                // Some(_) => (),
                None => (),
            }
        }
    }

    fn binary<T>(stack: &mut Stack, implementation: T) -> InterpretResult<()>
    where
        T: Fn(f64, f64) -> Value,
    {
        let b = stack.pop_number()?;
        let a = stack.pop_number()?;
        let res = implementation(a, b);
        stack.push(res);
        Ok(())
    }
}
