use crate::lexer::location::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    kind: Kind,
    loc: Location,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Ident(String),
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
