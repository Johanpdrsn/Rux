use crate::chunk::{Chunk, OpCode};
use crate::stack::Stack;
use crate::value::Value;

struct CallFrame<'a> {
    function: &'a Chunk,
    ip: usize,
}

impl<'a> CallFrame<'a> {
    pub fn new(func: &'a Chunk) -> Self {
        CallFrame {
            function: func,
            ip: 0,
        }
    }
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
#[derive(Debug)]
pub struct VM {
    pub stack: Stack,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: Stack::new(),
        }
    }

    pub fn run_main(&mut self, function: Chunk) -> InterpretResult<()> {
        self.run(function)
    }

    pub fn run(&mut self, function: Chunk) -> InterpretResult<()> {
        let mut frame = CallFrame::new(&function);

        loop {
            let op = frame.function.op_get(frame.ip);

            for val in self.stack.contents() {
                println!("[ {val} ]");
            }
            op.unwrap().disassemble(&frame.function, frame.ip);

            frame.ip += 1;

            match op {
                Some(OpCode::Return) => {
                    println!("{}", self.stack.pop().unwrap());
                    return Ok(());
                }
                Some(OpCode::Constant(iid)) => {
                    let constant = frame.function.read_constant(*iid);
                    self.stack.push(constant.clone());
                }
                Some(OpCode::Nil) => self.stack.push(Value::Nil),
                Some(OpCode::True) => self.stack.push(Value::Boolean(true)),
                Some(OpCode::False) => self.stack.push(Value::Boolean(false)),
                Some(OpCode::Negate) => {
                    let n = self.stack.pop_number()?;
                    let res = -n;
                    self.stack.push(Value::Number(res));
                }
                Some(OpCode::Add) => VM::binary(&mut self.stack, |a, b| Value::Number(a + b))?,
                Some(OpCode::Subtract) => VM::binary(&mut self.stack, |a, b| Value::Number(a - b))?,
                Some(OpCode::Multiply) => VM::binary(&mut self.stack, |a, b| Value::Number(a * b))?,
                Some(OpCode::Divide) => VM::binary(&mut self.stack, |a, b| Value::Number(a / b))?,
                Some(OpCode::Not) => {
                    let old = self.stack.pop()?;
                    let new = old.is_falsey();
                    self.stack.push(Value::Boolean(new));
                }
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
