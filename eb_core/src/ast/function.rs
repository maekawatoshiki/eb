use super::expr;
use crate::lexer::location::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    name: String,
    params: Vec<Param>,
    body: Vec<expr::Node>,
    loc: Location,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    name: String,
}

impl Node {
    pub fn new(name: String, params: Vec<Param>, body: Vec<expr::Node>, loc: Location) -> Self {
        Self {
            name,
            params,
            body,
            loc,
        }
    }
}

impl Param {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl From<Node> for super::Node {
    fn from(node: Node) -> Self {
        super::Node::Function(node)
    }
}
