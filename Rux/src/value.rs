use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => f.write_str("nil"),
            Value::Boolean(b) => f.write_str(&b.to_string()),
            Value::Number(n) => f.write_str(&n.to_string()),
        }
    }
}

impl Value {
    pub fn is_falsey(&self) -> bool {
        match self {
            Value::Boolean(b) => !b,
            Value::Nil => true,
            _ => false,
        }
    }
}
