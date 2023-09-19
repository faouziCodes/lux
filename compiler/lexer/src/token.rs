#[derive(PartialEq, Debug)]
pub enum TokenType {
    Semicolon,
    LCBracket,
    RCBracket,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Colon,
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
    I32,
    I16,
    I8,
    F32,
    F16,
    F8,
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
    pub line: usize,
}

impl Token {
    pub fn new(token_t: TokenType, token_v: String, line: usize) -> Self {
        Self {
            token_t,
            token_v,
            line,
        }
    }
}

impl TryFrom<&str> for Keyword {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "let" => Ok(Self::Let),
            "CONST" => Ok(Self::Const),
            "if" => Ok(Self::If),
            "fn" => Ok(Self::Fn),
            "else" => Ok(Self::Else),
            "for" => Ok(Self::For),
            "while" => Ok(Self::While),
            "i8" => Ok(Self::I8),
            "i16" => Ok(Self::I16),
            "i32" => Ok(Self::I32),
            "f8" => Ok(Self::F8),
            "f16" => Ok(Self::F16),
            "f32" => Ok(Self::F32),
            _ => Err("Not a keyword"),
        }
    }
}
