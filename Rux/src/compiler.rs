use crate::{
    scanner::Scanner,
    token::{TokenResult, TokenType},
};

#[derive(Debug)]
pub struct Compiler<'a> {
    scanner: Scanner<'a>,
    previous: TokenResult<'a>,
    current: TokenResult<'a>,
}

impl<'a> Compiler<'a> {
    pub fn from_source(source: &'a str) -> Self {
        Compiler {
            scanner: Scanner::new(source),
            previous: TokenResult::invalid(),
            current: TokenResult::invalid(),
        }
    }

    pub fn compile(&mut self) {
        self.advance();
    }

    fn advance(&mut self) {
        self.previous = self.current.clone();

        while self.current.token_type != TokenType::Eof {
            self.current = self.scanner.scan_token();

            println!("{:?}", self.current);
        }
    }
}
