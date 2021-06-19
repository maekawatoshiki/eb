use super::{expr, Context};
use crate::{
    ast::function as ast_func,
    lexer::token::{DelimKind, PunctKind},
};
use anyhow::Result;

pub fn parse(ctx: &mut Context) -> Result<ast_func::Node> {
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
    let body = expr::parse_body(ctx)?;
    Ok(ast_func::Node::new(ident, params, body))
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

        if ctx.skip_punct(PunctKind::Comma) {
            continue;
        }

        ctx.expect_close_delim(DelimKind::Paren)?;

        return Ok(params);
    }
}

#[cfg(test)]
mod test {
    extern crate insta;
    use super::*;
    use crate::lexer::{source::Source, tokenize};

    #[test]
    fn parse1() {
        let source = Source::String(r#"func f(): ;;"#.to_string());
        let mut ctx = Context::new(tokenize(&source));
        insta::assert_debug_snapshot!(parse(&mut ctx).expect("fail to parse"));
    }

    #[test]
    fn parse2() {
        let source = Source::String(r#"func f(x): ;;"#.to_string());
        let mut ctx = Context::new(tokenize(&source));
        insta::assert_debug_snapshot!(parse(&mut ctx).expect("fail to parse"));
    }

    #[test]
    fn parse3() {
        let source = Source::String(r#"func f(x, y): ;;"#.to_string());
        let mut ctx = Context::new(tokenize(&source));
        insta::assert_debug_snapshot!(parse(&mut ctx).expect("fail to parse"));
    }

    #[test]
    fn parse4() {
        let source = Source::String(r#"func f(x): x;;"#.to_string());
        let mut ctx = Context::new(tokenize(&source));
        insta::assert_debug_snapshot!(parse(&mut ctx).expect("fail to parse"));
    }

    #[test]
    fn parse5() {
        let source = Source::String(r#"func f(x): x;;"#.to_string());
        let mut ctx = Context::new(tokenize(&source));
        insta::assert_debug_snapshot!(parse(&mut ctx).expect("fail to parse"));
    }

    #[test]
    fn parse6() {
        let source = Source::String(
            r#"
            func fact(x): 
                if x == 1:
                    return 1 ;;
                x * fact(x - 1) ;;
                "#
            .to_string(),
        );
        let mut ctx = Context::new(tokenize(&source));
        insta::assert_debug_snapshot!(parse(&mut ctx).expect("fail to parse"));
    }
}
