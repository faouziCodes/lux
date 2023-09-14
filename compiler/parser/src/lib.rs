mod parse_var;

use lexer::token::{Token, TokenType};
use luxast::span::Span;

pub struct Parser<'t> {
    tokens: &'t Vec<Token>,
    pos: usize,
}

impl<'t, 'p> Parser<'t> {
    fn new(tokens: &'t Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn current_token(&self) -> Option<&'t Token> {
        self.tokens.get(self.pos)
    }

    // Returns a span starting from the current parser position up and including the delim;
    fn gen_span(&mut self, delim: &TokenType) -> Option<Span> {
        let current_token = self.current_token()?;
        let mut span = Span::new(current_token.line, self.pos);

        while let Some(token) = self.next() {
            span.set_end(self.pos);
            match token {
                t if  t == delim => return Some(span),
                _ => continue,
            }
        }

        None
    }
}

impl<'t, 'p> Iterator for Parser<'t> {
    type Item = &'t Token;
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.tokens.get(self.pos)?;
        self.pos += 1;
        Some(token)
    }
}


trait Parsable {
    fn parse<'t>(&mut self, parser: &'t Parser<'t>);
}