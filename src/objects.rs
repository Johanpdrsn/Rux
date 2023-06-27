use std::fmt::Display;

#[derive(Debug)]
pub struct StringObject {
    pub value: String,
}

impl StringObject {
    pub fn new(str: &str) -> StringObject {
        StringObject {
            value: String::from(str),
        }
    }
    pub fn from_owned(str: String) -> StringObject {
        StringObject { value: str }
    }
}

impl Display for StringObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.value)
    }
}
