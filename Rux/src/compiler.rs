use crate::{
    chunk::Chunk,
    chunk::OpCode,
    precedence::Precedence,
    scanner::Scanner,
    token::{TokenResult, TokenType},
};

#[derive(Debug)]
pub struct Compiler<'a> {
    scanner: Scanner<'a>,
    previous: TokenResult<'a>,
    current: TokenResult<'a>,
    pub had_error: bool,
    panic_mode: bool,
}

impl<'a> Compiler<'a> {
    pub fn from_source(source: &'a str) -> Self {
        Compiler {
            scanner: Scanner::new(source),
            previous: TokenResult::invalid(),
            current: TokenResult::invalid(),
            had_error: false,
            panic_mode: false,
        }
    }

    pub fn compile(&mut self) -> Chunk {
        let mut frame = Chunk::new();
        self.advance();

        if !self.matches(TokenType::Eof){
            self.expression(&mut frame);
        }

        self.emit_return(&mut frame);

        #[cfg(feature = "debug_print_code")]
        if !self.had_error {
            frame.disassemble("code");
        }

        frame
    }

    fn advance(&mut self) {
        self.previous = self.current.clone();

        loop {
            self.current = self.scanner.scan_token();
            match &self.current.data.clone() {
                Ok(_) => break,
                Err(message) => self.error_at_current(&message),
            }
        }
    }

    fn consume(&mut self, expected_type: TokenType, message: &str) {
        if expected_type == self.current.token_type {
            self.advance()
        } else {
            self.error_at_current(message)
        }
    }

    fn matches(&mut self, expected: TokenType) -> bool {
        if self.current.token_type == expected {
            self.advance();
            true
        } else {
            false
        }
    }

    fn emit_return(&self, frame: &mut Chunk) {
        frame.emit(crate::chunk::OpCode::Return)
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(self.current.line, message)
    }
    fn error_at(&mut self, line: i32, message: &str) {
        if !self.panic_mode {
            self.panic_mode = true;
            println!(
                "
                [line {}] Error: {}
                Compiler state: {:#?}
                ",
                line, message, self
            );
            self.had_error = true;
        }
    }

    fn expression(&mut self, frame: &mut Chunk) {
        self.parse_precedence(Precedence::Assignment, frame)
    }

    fn number(&mut self, frame: &mut Chunk) {
        let data = self.previous.data.as_ref().unwrap();
        let val = data.lexeme.parse::<f64>().unwrap();
        frame.emit_constant(crate::value::Value::Number(val))
    }

    fn grouping(&mut self, frame: &mut Chunk) {
        self.expression(frame);
        self.consume(TokenType::RightParen, "Expect ')' after expression.")
    }

    fn unary(&mut self, frame: &mut Chunk) {
        let operator_type = self.previous.token_type;

        self.parse_precedence(Precedence::Unary, frame);

        match operator_type {
            TokenType::Minus => frame.emit(OpCode::Negate),
            TokenType::Bang => frame.emit(OpCode::Not),
            _ => (),
        }
    }

    fn binary(&mut self, frame: &mut Chunk) {
        let operator_type = self.previous.token_type;
        self.parse_precedence(Self::get_precedence(operator_type).next(), frame);
        match operator_type {
            TokenType::Plus => frame.emit(OpCode::Add),
            TokenType::Minus => frame.emit(OpCode::Subtract),
            TokenType::Star => frame.emit(OpCode::Multiply),
            TokenType::Slash => frame.emit(OpCode::Divide),
            _ => (),
        }
    }

    fn parse_precedence(&mut self, precedence: Precedence, frame: &mut Chunk) {
        self.advance();

        self.prefix_rule(self.previous.token_type, frame);

        while precedence <= Self::get_precedence(self.current.token_type) {
            self.advance();
            self.infix_rule(self.previous.token_type, frame);
        }
    }

    fn get_precedence(operator_type: TokenType) -> Precedence {
        match operator_type {
            TokenType::Minus => Precedence::Term,
            TokenType::Plus => Precedence::Term,
            TokenType::Slash => Precedence::Factor,
            TokenType::Star => Precedence::Factor,
            TokenType::BangEqual => Precedence::Equality,
            TokenType::EqualEqual => Precedence::Equality,
            TokenType::Greater => Precedence::Comparison,
            TokenType::GreaterEqual => Precedence::Comparison,
            TokenType::Less => Precedence::Comparison,
            TokenType::LessEqual => Precedence::Comparison,
            TokenType::And => Precedence::And,
            TokenType::Or => Precedence::Or,
            TokenType::LeftParen => Precedence::Call,
            _ => Precedence::None,
        }
    }

    fn prefix_rule(&mut self, operator_type: TokenType, frame: &mut Chunk) {
        match operator_type {
            TokenType::LeftParen => self.grouping(frame),
            TokenType::Minus => self.unary(frame),
            TokenType::Number => self.number(frame),
            TokenType::Bang => self.unary(frame),
            tt => panic!("Expected expresion, got {:?}", tt),
        }
    }

    fn infix_rule(&mut self, operator_type: TokenType, frame: &mut Chunk) {
        match operator_type {
            TokenType::Minus => self.binary(frame),
            TokenType::Plus => self.binary(frame),
            TokenType::Slash => self.binary(frame),
            TokenType::Star => self.binary(frame),
            TokenType::BangEqual => self.binary(frame),
            TokenType::EqualEqual => self.binary(frame),
            TokenType::Greater => self.binary(frame),
            TokenType::GreaterEqual => self.binary(frame),
            TokenType::Less => self.binary(frame),
            TokenType::LessEqual => self.binary(frame),
            _ => (), //panic!("Expect expresion"),
        }
    }
}
