#[derive(PartialEq, Debug)]
pub enum TokenType {
    Semicolon,
    LCBracket,
    RCBracket,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Number,
    Ident,
    String,
    Kw(Keyword),
    Op(Operators),
    Whitespace,
    Invalid,
}

#[derive(PartialEq, Debug)]
pub enum Keyword {
    Const,
    Let,
    Fn,
    If,
    Else,
    For,
    While,
}

#[derive(PartialEq, Debug)]
pub enum Operators {
    Eq,
    EqEq,
    More,
    MoreEq,
    Less,
    LessEq,
    NEq,
    Bang,
    Plus,
    Min,
    Times,
}

#[derive(Debug)]
pub struct Token {
    pub token_t: TokenType,
    pub token_v: String,
}

impl Token {
    pub fn new(token_t: TokenType, token_v: String) -> Self {
        Self { token_t, token_v }
    }
}
