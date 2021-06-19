use super::expr;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    name: String,
    params: Vec<Param>,
    body: Vec<expr::Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    name: String,
}

impl Node {
    pub fn new(name: String, params: Vec<Param>, body: Vec<expr::Node>) -> Self {
        Self { name, params, body }
    }
}

impl Param {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
