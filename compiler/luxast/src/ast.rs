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

pub struct Var {
    id: Option<Ident>,
    ty: Option<TypeOf>,
    expr: Option<Box<Node>>,
}

impl Var {
    pub fn new(id: Option<Ident>, ty: Option<TypeOf>, expr: Option<Box<Node>>) -> Self {
        Self { id, ty, expr }
    }
}

pub struct Stmt {
    loc: Span,
    stmt: Vec<Node>,
}

pub struct Func {
    id: Ident,
    params: Box<Node>,
    body: Box<Node>,
}

pub struct Type {
    ty: Option<TypeOf>,
    is_ptr: bool,
}

impl Type {
    pub fn new(ty: Option<TypeOf>, is_ptr: bool) -> Self {
        Self { ty, is_ptr }
    }
}

pub struct Typedef {
    id: Ident,
    fields: HashMap<Ident, TypeOf>,
}

pub struct Expr {
    loc: Span,
    val: Token,
    lhs: Box<Node>,
    rhs: Box<Node>,
}

pub enum Node {
    Program { loc: Span, nodes: Vec<Node> },
    Expr(Expr),
    Stmt(Stmt),
    Var(Var),
    Func(Func),
    Type(Type),
    Typedef(Typedef),
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
