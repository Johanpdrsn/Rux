#[derive(Clone, Debug)]
pub struct TokenResult<'a> {
    pub line: i32,
    pub token_type: TokenType,
    pub data: Result<Token<'a>, String>,
}

impl<'a> TokenResult<'a> {
    pub fn invalid() -> Self {
        TokenResult {
            line: -1,
            token_type: TokenType::Error,
            data: Err(String::from("Invalid")),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token<'a> {
    pub start: usize,
    pub stop: usize,
    pub lexeme: &'a str,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals.
    Identifier,
    String,
    Number,

    // keywords.
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    Eof,
}
