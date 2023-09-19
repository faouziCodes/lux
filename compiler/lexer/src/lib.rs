#[cfg(test)]
mod tests;
pub mod token;

use token::{Keyword, Operators, Token, TokenType};

pub struct Lexer {
    position: usize,
    curr_line: usize,
    input: Vec<char>,
    buffer: String,
}

pub trait Lex {
    fn new(input: &str) -> Self;
    fn tokenize(&mut self) -> Vec<Token>;
    fn next_token(&mut self) -> Option<Token>;
}

impl Lexer {
    fn peak(&mut self) -> Option<&char> {
        self.input.get(self.position)
    }

    /// Clears the buffer and returns the cleared value
    fn clear_buffer<'a>(&mut self) -> String {
        let buffer = self.buffer.clone();
        self.buffer.clear();
        buffer
    }
}

impl Lex for Lexer {
    fn new(input: &str) -> Self {
        let input = input.chars().collect();
        return Self {
            position: 0,
            curr_line: 0,
            input,
            buffer: String::new(),
        };
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            if token.token_t == TokenType::Whitespace {
                continue;
            }
            tokens.push(token);
        }
        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        let nchar = self.next()?;

        let token_t = match nchar {
            c if c.is_whitespace() || c == '\n' => TokenType::Whitespace,
            c if c.is_numeric() => {
                while let Some(c) = self.peak() {
                    if !c.is_numeric() {
                        break;
                    }
                    self.next();
                }
                TokenType::Number
            }
            c if c.is_alphabetic() => {
                while let Some(c) = self.peak() {
                    if !c.is_alphabetic() {
                        break;
                    }
                    self.next();
                }
                TokenType::Ident
            }
            '"' => {
                self.clear_buffer();
                while let Some(c) = self.next() {
                    if c == '"' {
                        self.buffer.pop();
                        break;
                    }
                }
                TokenType::String
            }
            '!' if self.peak() == Some(&'=') => {
                self.next();
                TokenType::Op(Operators::NEq)
            }
            '=' if self.peak() == Some(&'=') => {
                self.next();
                TokenType::Op(Operators::EqEq)
            }
            '>' if self.peak() == Some(&'=') => {
                self.next();
                TokenType::Op(Operators::MoreEq)
            }
            '<' if self.peak() == Some(&'=') => {
                self.next();
                TokenType::Op(Operators::LessEq)
            }
            '[' => TokenType::LBracket,
            ']' => TokenType::RBracket,
            '{' => TokenType::LCBracket,
            '}' => TokenType::RCBracket,
            '(' => TokenType::LBrace,
            ')' => TokenType::RBrace,
            ':' => TokenType::Colon,
            ';' => TokenType::Semicolon,
            '=' => TokenType::Op(Operators::Eq),
            '>' => TokenType::Op(Operators::More),
            '<' => TokenType::Op(Operators::Less),
            '!' => TokenType::Op(Operators::Bang),
            '+' => TokenType::Op(Operators::Plus),
            '-' => TokenType::Op(Operators::Min),
            '*' => TokenType::Op(Operators::Times),
            _ => TokenType::Invalid,
        };

        let token_t = if token_t == TokenType::Ident {
            match Keyword::try_from(self.buffer.as_str()) {
                Ok(kw) => TokenType::Kw(kw),
                Err(_) => token_t,
            }
        } else {
            token_t
        };

        Some(Token::new(token_t, self.clear_buffer(), self.curr_line))
    }
}

impl Iterator for Lexer {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.input.get(self.position)?;
        self.position += 1;
        if *current == '\n' {
            self.curr_line += 1;
        }

        let inp = *current;
        self.buffer.push(inp);

        Some(inp)
    }
}
