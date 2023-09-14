use std::collections::HashMap;

use lexer::token::Token;

use crate::span::Span;

/// [`Ident`] used to bind a name to a certaint node
pub struct Ident {
    pos: Span,
    val: String,
}

pub enum IntType {
    I8,
    I32,
    I64,
    UI8,
    UI32,
    UI64,
}

pub enum FloatType {
    F8,
    F32,
    F64,
}

pub enum TypeOf {
    Array(Box<TypeOf>),
    Int(IntType),
    Float(FloatType),
    TypeDef { id: Ident },
}

pub enum Node {
    Program {
        loc: Span,
        nodes: Vec<Node>,
    },
    Expr {
        loc: Span,
        val: Token,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Stmt {
        loc: Span,
        stmt: Vec<Node>,
    },
    Var {
        id: Ident,
        ty: TypeOf,
        expr: Box<Node>,
    },
    Func {
        id: Ident,
        params: Box<Node>,
        body: Box<Node>,
    },
    Type {
        ty: TypeOf,
        ptr: bool,
    },
    Typedef {
        id: Ident,
        fields: HashMap<Ident, TypeOf>,
    },
}

pub struct Ast {
    span: Span,
    entry: Node,
}

impl Ast {
    fn new(span: Span) -> Self {
        Self {
            span,
            entry: Node::Program {
                loc: span,
                nodes: Vec::new(),
            },
        }
    }
}
