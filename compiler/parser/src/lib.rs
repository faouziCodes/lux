use lexer::token::{Token, TokenType};
use luxast::span::Span;

pub struct Parser<'t> {
    tokens: &'t Vec<Token>,
    position: usize,
    current_line: usize
}

impl<'t> Parser<'t> {
    pub fn new(tokens: &'t Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
            current_line: 0,
        }
    }

    /// Iterates incrementing the position of the parser until a certaint delimiter
    /// Returns a Span of the position iteration started up until the position it found the
    /// delimiter.
    pub fn pos_until(&mut self, delimiter: &TokenType) -> Span {
        let mut span = Span::new(self.current_line, self.position);
        while let Some(token) = self.next() {
            match token {
                token_type if &token_type.token_t == delimiter => {
                    span.set_end(self.position);
                    break;
                }
                _ => continue,
            }
        }
        span
    }
}

impl<'t> Iterator for Parser<'t> {
    type Item = &'t Token;
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.tokens.get(self.position)?;
        self.position += 1;
        self.current_line =  token.line;
        Some(&token)
    }
}
