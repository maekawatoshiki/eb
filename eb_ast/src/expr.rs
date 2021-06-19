use super::function;
use lexer::location::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    kind: Kind,
    loc: Location,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Int(i64),
    Ident(String),
    Function(Box<function::Node>),
    BinOp(BinOpKind, Box<Node>, Box<Node>),
    Call(Box<Node>, Vec<Node>),
    If(Box<Node>, Box<Node>, Option<Box<Node>>),
    Exprs(Vec<Node>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Neq,
}

impl Node {
    pub fn new(kind: Kind, loc: Location) -> Self {
        Self { kind, loc }
    }
}
