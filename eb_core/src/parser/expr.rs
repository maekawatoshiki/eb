use super::Context;
use crate::ast::expr;
use anyhow::Result;

pub fn parse(ctx: &mut Context) -> Result<expr::Node> {
    let loc = ctx.cur_loc()?;
    let ident = ctx
        .expect_any_ident()?
        .kind()
        .as_ident()
        .unwrap()
        .to_string();
    Ok(expr::Node::new(expr::Kind::Ident(ident), loc))
}
