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

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn params(&self) -> &[Param] {
        &self.params
    }

    pub fn body(&self) -> &[expr::Node] {
        &self.body
    }
}

impl Param {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
