use super::Context;
use anyhow::Result;

pub fn parse(ctx: &mut Context) -> Result<()> {
    ctx.expect_keyword("func")?;
    let ident = ctx.expect_any_ident()?;
    println!("{}", ident.kind().as_ident().unwrap());
    Ok(())
}
