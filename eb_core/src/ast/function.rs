use crate::lexer::location::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    name: String,
    loc: Location,
}

impl Node {
    pub fn new(name: String, loc: Location) -> Self {
        Self { name, loc }
    }
}

impl From<Node> for super::Node {
    fn from(node: Node) -> Self {
        super::Node::Function(node)
    }
}
