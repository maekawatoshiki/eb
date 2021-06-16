use super::{expr, Context};
use crate::{
    ast::{expr as ast_expr, function as ast_func},
    lexer::token::{DelimKind, PunctKind},
};
use anyhow::Result;

pub fn parse(ctx: &mut Context) -> Result<ast_func::Node> {
    let loc = ctx.cur_loc()?;
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
    let body = parse_body(ctx)?;
    Ok(ast_func::Node::new(ident, params, body, loc))
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

fn parse_body(ctx: &mut Context) -> Result<Vec<ast_expr::Node>> {
    if ctx.skip_punct(PunctKind::DoubleSemicolon) {
        return Ok(vec![]);
    }

    let mut body = vec![];

    loop {
        body.push(expr::parse(ctx)?);
        if ctx.skip_punct(PunctKind::DoubleSemicolon) {
            return Ok(body);
        }
    }
}

#[test]
fn parse1() {
    // use location::Location;
    use crate::ast::function as ast_func;
    use crate::lexer::{location::Location, source::Source, tokenize};

    let source = Source::String(r#"func f(): ;;"#.to_string());
    let mut ctx = Context::new(tokenize(&source));
    assert_eq!(
        parse(&mut ctx).expect("fail to parse"),
        ast_func::Node::new("f".to_string(), vec![], vec![], Location(0)).into()
    );
}

#[test]
fn parse2() {
    use crate::ast::function as ast_func;
    use crate::lexer::{location::Location, source::Source, tokenize};

    let source = Source::String(r#"func f(x): ;;"#.to_string());
    let mut ctx = Context::new(tokenize(&source));
    assert_eq!(
        parse(&mut ctx).expect("fail to parse"),
        ast_func::Node::new(
            "f".to_string(),
            vec![ast_func::Param::new("x".to_string())],
            vec![],
            Location(0)
        )
        .into()
    );

    let source = Source::String(r#"func f(x y): ;;"#.to_string());
    let mut ctx = Context::new(tokenize(&source));
    assert_eq!(
        parse(&mut ctx).expect("fail to parse"),
        ast_func::Node::new(
            "f".to_string(),
            vec![
                ast_func::Param::new("x".to_string()),
                ast_func::Param::new("y".to_string())
            ],
            vec![],
            Location(0)
        )
        .into()
    );
}

#[test]
fn parse3() {
    use crate::ast::{expr as ast_expr, function as ast_func};
    use crate::lexer::{location::Location, source::Source, tokenize};

    let source = Source::String(r#"func f(x): x;;"#.to_string());
    let mut ctx = Context::new(tokenize(&source));
    assert_eq!(
        parse(&mut ctx).expect("fail to parse"),
        ast_func::Node::new(
            "f".to_string(),
            vec![ast_func::Param::new("x".to_string())],
            vec![ast_expr::Node::new(
                ast_expr::Kind::Ident("x".to_string()),
                Location(10)
            )],
            Location(0)
        )
        .into()
    );
}
