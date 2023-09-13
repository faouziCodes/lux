use crate::span::Span;

pub enum NodeTypes {
    Function,
    Call,
    Arithmetic,
    Compare,
    Assign,
    While,
    Ident,
    If,
    Else,
    More,
    Less,
    Add,
    Min,
    And,
    Or,
}

pub enum NodeValues {
    Int(i64),
    Float(u64),
    String(String),
    Ident(String),
    Invalid(String),
    Call { calls: String, arguments: Vec<Self> },
}

pub enum Node {
    Program {
        body: Vec<Node>,
    },
    Node {
        span: Span,
        nt: NodeTypes,
        lhs: Option<Box<Node>>,
        rhs: Option<Box<Node>>,
    },
    Stmt {
        value: Vec<Node>,
    },
    Declaration {
        dt: NodeTypes,
        value: Box<Node>,
    },
    Expression {
        et: NodeTypes,
        value: Box<Node>,
    },
    ConditionalStmt {
        ct: NodeTypes,
        if_do: Box<Node>,
        else_if_do: Vec<Node>,
        else_do: Option<Box<Node>>,
    },
    Value(NodeValues),
}

impl Default for Node {
    fn default() -> Self {
        Self::Program { body: Vec::new() }
    }
}
