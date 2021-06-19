use super::FunctionContext;

#[derive(Debug, Clone)]
pub enum Value {
    Func(Box<FunctionContext>),
    Bool(bool),
    Int(i64),
    String(String),
    Nil,
}
