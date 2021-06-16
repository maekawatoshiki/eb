use super::Context;
use crate::lexer::token::{DelimKind, PunctKind};
use anyhow::Result;

pub fn parse(ctx: &mut Context) -> Result<()> {
    ctx.expect_keyword("func")?;
    let ident = ctx.expect_any_ident()?;
    ctx.expect_open_delim(DelimKind::Paren)?;
    ctx.expect_close_delim(DelimKind::Paren)?;
    ctx.expect_punct(PunctKind::Colon)?;
    // println!("{}", ident.kind().as_ident().unwrap());
    Ok(())
}
