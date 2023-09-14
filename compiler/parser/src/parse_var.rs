use luxast::ast::Var;

use crate::Parsable;

impl Parsable for Var {
    fn parse<'t>(&mut self, parser: &'t crate::Parser<'t>) {}
}

