mod parse_var;
mod errors;

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
                t if  &t.token_t == delim => return Some(span),
                _ => continue,
            }
        }

        None
    }

    fn iter_span(&mut self, span: Span) -> IterSpan {
        let iter_tokens = &self.tokens[span.start..span.end];
        IterSpan {
            pos: 0,
            iter: iter_tokens 
        }
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
    /// fn parse defines how to parse the structure, gives the implementation access to 
    /// the parser structure allowing it to creates spans and iterate over the tokens
    /// within a span
    fn parse<'t>(&mut self, parser: &'t mut Parser<'t>);
}

pub struct IterSpan<'st> {
    pos: usize,
    iter: &'st [Token]
}

impl<'st> Iterator for IterSpan<'st> {
    type Item = &'st Token;

    fn next(&mut self) -> Option<Self::Item> { 
        let token = self.iter.get(self.pos)?;
        self.pos += 1;
        Some(token)
    }
}
