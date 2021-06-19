use super::expr;
use anyhow::Result;
use ast::function as func;
use vm::inst::{Code, Inst};

pub struct Context {
    pub code: Code,
    pub children: Vec<Context>,
}

pub fn visit(ctx: &mut Context, func: &func::Node) -> Result<()> {
    for expr in func.body() {
        expr::visit(ctx, expr)?;
    }
    todo!()
}

impl Default for Context {
    fn default() -> Self {
        Self {
            code: Code(vec![]),
            children: vec![],
        }
    }
}

impl Context {
    pub fn push(&mut self, inst: Inst) {
        self.code.0.push(inst)
    }

    pub fn add_child(&mut self, ctx: Self) {
        self.children.push(ctx)
    }
}
