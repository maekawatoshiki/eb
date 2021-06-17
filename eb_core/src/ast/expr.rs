use crate::lexer::{location::Location, token::PunctKind};

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    kind: Kind,
    loc: Location,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Int(i64),
    Ident(String),
    BinOp(BinOpKind, Box<Node>, Box<Node>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

impl Node {
    pub fn new(kind: Kind, loc: Location) -> Self {
        Self { kind, loc }
    }
}

impl From<Node> for super::Node {
    fn from(node: Node) -> Self {
        super::Node::Expr(node)
    }
}

impl From<PunctKind> for BinOpKind {
    fn from(p: PunctKind) -> Self {
        match p {
            PunctKind::Plus => Self::Add,
            PunctKind::Minus => Self::Sub,
            PunctKind::Eq => Self::Eq,
            _ => panic!(),
        }
    }
}
