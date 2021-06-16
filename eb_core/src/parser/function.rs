use super::{Context, Error};
use crate::{
    ast::{function, Node},
    lexer::token::{DelimKind, PunctKind},
};
use anyhow::Result;

pub fn parse(ctx: &mut Context) -> Result<Node> {
    let loc = ctx.peek().map_or(Err(Error::EOF), |t| Ok(*t.loc()))?;
    ctx.expect_keyword("func")?;
    let ident = ctx
        .expect_any_ident()?
        .kind()
        .as_ident()
        .unwrap()
        .to_string();
    ctx.expect_open_delim(DelimKind::Paren)?;
    ctx.expect_close_delim(DelimKind::Paren)?;
    ctx.expect_punct(PunctKind::Colon)?;
    ctx.expect_punct(PunctKind::DoubleSemicolon)?;
    Ok(function::Node::new(ident, loc).into())
}
