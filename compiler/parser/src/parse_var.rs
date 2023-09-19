use lexer::token::TokenType;
use luxast::ast::Var;
use crate::Parsable;

impl Parsable for Var {
    fn parse<'t>(&mut self, parser: &'t mut crate::Parser<'t>) {
        let Some(var_span) = parser.gen_span(&TokenType::Semicolon) else {
            return;
        };

        let var = Var::new(None, None, None);
        let mut iter = parser.iter_span(var_span);

        let let_after = match iter.next() {
            Some(token) => {
                let ident = iter.next();
                ident
            }
            _ => panic!("I should probably implement this.")
        };
         
        let ident_value = match let_after {
           Some(token) =>{
                token.token_v
           }
           None => panic!("I should probably implement this.")
        }
    } 
}
