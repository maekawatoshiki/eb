pub mod function;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Function(function::Node),
}
