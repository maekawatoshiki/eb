use super::{Context, Error};
use crate::{
    ast::{function as ast_func, Node},
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
    let params = parse_parameters(ctx)?;
    ctx.expect_punct(PunctKind::Colon)?;
    ctx.expect_punct(PunctKind::DoubleSemicolon)?;
    Ok(ast_func::Node::new(ident, params, loc).into())
}

fn parse_parameters(ctx: &mut Context) -> Result<Vec<ast_func::Param>> {
    if ctx.skip_close_delim(DelimKind::Paren) {
        return Ok(vec![]);
    }
    let mut params = vec![];

    loop {
        let param = ctx
            .expect_any_ident()?
            .kind()
            .as_ident()
            .unwrap()
            .to_string();
        params.push(ast_func::Param::new(param));

        if ctx.skip_close_delim(DelimKind::Paren) {
            return Ok(params);
        }
    }
}
