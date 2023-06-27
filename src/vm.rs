use std::rc::Rc;
use std::{fmt::Display, ops::Neg};

use crate::chunk::{Chunk, OpCode};
use crate::objects::StringObject;
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

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::NoMoreOperations(ip) => f.write_fmt(format_args!(
                "The VM was halted because there were no more operations at the ip {}",
                ip
            )),
            RuntimeError::Other(str) => f.write_str(str),
        }
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

    pub fn run_main(&mut self, function: &Chunk) -> InterpretResult<()> {
        self.run(function)
    }

    pub fn run(&mut self, function: &Chunk) -> InterpretResult<()> {
        let mut frame = CallFrame::new(&function);
        let code = function.code();
        loop {
            let op = code
                .get(frame.ip)
                .ok_or(RuntimeError::NoMoreOperations(frame.ip))?;

            // for val in self.stack.contents() {
            //     println!("[ {val} ]");
            // }

            frame.ip += 1;

            match op {
                OpCode::Return => {
                    println!("{}", self.stack.pop().unwrap());
                    return Ok(());
                }
                OpCode::Constant(iid) => {
                    let constant = frame.function.read_constant(*iid);
                    self.stack.push(constant.clone());
                }
                OpCode::Nil => self.stack.push(Value::Nil),
                OpCode::True => self.stack.push(Value::Boolean(true)),
                OpCode::False => self.stack.push(Value::Boolean(false)),
                OpCode::Negate => {
                    let n = self.stack.pop_number()?;
                    self.stack.push(Value::Number(n.neg()));
                }
                OpCode::Add => match self.stack.peek()? {
                    Value::Number(_) => VM::binary(&mut self.stack, |a, b| Value::Number(a + b))?,
                    Value::String(_) => {
                        let b = self.stack.pop_string()?;
                        let a = self.stack.pop_string()?;
                        self.stack
                            .push(Value::String(Rc::from(StringObject::from_owned(format!(
                                "{}{}",
                                a.value, b.value
                            )))));
                    }
                    v => Err(RuntimeError::new(&format!("Can't add operand {:?}", v)))?,
                },
                OpCode::Subtract => VM::binary(&mut self.stack, |a, b| Value::Number(a - b))?,
                OpCode::Multiply => VM::binary(&mut self.stack, |a, b| Value::Number(a * b))?,
                OpCode::Divide => VM::binary(&mut self.stack, |a, b| Value::Number(a / b))?,
                OpCode::Not => {
                    let old = self.stack.pop()?;
                    let new = old.is_falsey();
                    self.stack.push(Value::Boolean(new));
                }
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

#[cfg(test)]
mod tests {
    use super::{CallFrame, VM};
    use crate::{
        chunk::{Chunk, OpCode},
        value::Value,
        vm::RuntimeError,
    };

    #[test]
    fn constants() {
        let mut chunk = Chunk::new();
        chunk.emit_many(&mut vec![OpCode::Constant(0), OpCode::True, OpCode::Nil]);
        chunk.add_constant(Value::Number(2.0));

        let mut function = CallFrame::new(&chunk);
        assert_stack(
            &mut function,
            vec![Value::Number(2.0), Value::Boolean(true), Value::Nil],
        )
    }

    fn assert_stack(function: &mut CallFrame, stack: Vec<Value>) {
        let mut vm = VM::new();
        match vm.run(function.function) {
            Ok(_) => panic!("Expected the VM to halt but it didn't"),
            Err(RuntimeError::NoMoreOperations(_)) => {
                assert_eq!(
                    vm.stack.contents(),
                    &stack,
                    "Stack contents are not the same"
                )
            }
            Err(other) => panic!(
                "Expected the VM to halt but another error happened: {}",
                other
            ),
        }
    }
}
