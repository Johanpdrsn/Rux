#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum Precedence {
    None,
    Assignment, // =
    Or,         // or
    And,        // and
    Equality,   // == !=
    Comparison, // < > <= >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // . ()
    Primary,
}

impl TryFrom<i32> for Precedence {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == Precedence::None as i32 => Ok(Precedence::None),
            x if x == Precedence::Assignment as i32 => Ok(Precedence::Assignment),
            x if x == Precedence::Or as i32 => Ok(Precedence::Or),
            x if x == Precedence::And as i32 => Ok(Precedence::And),
            x if x == Precedence::Equality as i32 => Ok(Precedence::Equality),
            x if x == Precedence::Comparison as i32 => Ok(Precedence::Comparison),
            x if x == Precedence::Term as i32 => Ok(Precedence::Term),
            x if x == Precedence::Factor as i32 => Ok(Precedence::Factor),
            x if x == Precedence::Unary as i32 => Ok(Precedence::Unary),
            x if x == Precedence::Call as i32 => Ok(Precedence::Call),
            x if x == Precedence::Primary as i32 => Ok(Precedence::Primary),
            _ => Err(()),
        }
    }
}

impl Precedence {
    pub fn next(&self) -> Precedence {
        Precedence::try_from((*self as i32 + 1).min(10)).unwrap()
    }
}
