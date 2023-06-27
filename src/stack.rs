use crate::{
    objects::StringObject,
    value::Value,
    vm::{InterpretResult, RuntimeError},
};
use std::{
    fmt::{Display, Formatter, Result},
    rc::Rc,
};
#[derive(Debug)]
pub struct Stack {
    values: Vec<Value>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { values: vec![] }
    }

    pub fn push(&mut self, value: Value) {
        self.values.push(value)
    }

    pub fn pop(&mut self) -> InterpretResult<Value> {
        self.values
            .pop()
            .ok_or(RuntimeError::new("Can't pop emty stack"))
    }

    pub fn pop_number(&mut self) -> InterpretResult<f64> {
        match self.pop()? {
            Value::Number(n) => Ok(n),
            v => Err(RuntimeError::new(&format!(
                "Expected to pop a number but found '{}'.\n{}",
                v, self
            ))),
        }
    }

    pub fn pop_string(&mut self) -> InterpretResult<Rc<StringObject>> {
        match self.pop()? {
            Value::String(s) => Ok(s),
            v => Err(RuntimeError::new(&format!(
                "Expected to pop a string but found '{}'.\n{}",
                v, self
            ))),
        }
    }

    pub fn peek(&mut self) -> InterpretResult<&Value> {
        self.values
            .last()
            .ok_or(RuntimeError::new("Tried to peek empty stack"))
    }

    pub fn contents(&self) -> &Vec<Value> {
        &&self.values
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.contents().len() == 0 {
            f.write_str("        <empty stack>\n")?;
        } else {
            for (i, val) in self.values.iter().enumerate() {
                f.write_str(&format!("    [{i}]  {val}\n"))?;
            }
        }
        Ok(())
    }
}
