use lexer::token::{TokenType};
use luxast::ast::Type;
use crate::Parsable;

impl Parsable for Type {
    fn parse<'t>(&mut self, parser: &'t mut crate::Parser<'t>) -> Result<Self, ()> {
        let type_def = Self::new(None, false);

        let type_of = match parser.peak() {
            Some(token) => match token.token_t {
                TokenType::Colon => {
                    parser.next();
                }
                TokenType::Kw(kw) => match kw {
                    
                }
                _ => return Err(())
            },
            None => return Err(()),
        };

        Ok(type_def)
    }
}
