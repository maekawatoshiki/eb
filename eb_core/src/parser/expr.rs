use super::{function, Context, Error};
use crate::{
    ast::expr,
    lexer::token::{DelimKind, PunctKind, TokenKind},
};
use anyhow::Result;

pub fn parse(ctx: &mut Context) -> Result<expr::Node> {
    parse_binop_add_sub(ctx)
}

pub fn parse_binop_add_sub(ctx: &mut Context) -> Result<expr::Node> {
    let mut lhs = parse_binop_mul_div(ctx)?;
    loop {
        let loc = ctx.cur_loc();

        let plus = ctx.skip_punct(PunctKind::Plus);
        let minus = ctx.skip_punct(PunctKind::Minus);

        if !plus && !minus {
            break;
        }

        let loc = loc?;
        let rhs = parse_binop_mul_div(ctx)?;

        lhs = expr::Node::new(
            expr::Kind::BinOp(
                if plus {
                    expr::BinOpKind::Add
                } else {
                    expr::BinOpKind::Sub
                },
                Box::new(lhs.clone()),
                Box::new(rhs),
            ),
            loc,
        );
    }
    Ok(lhs)
}

pub fn parse_binop_mul_div(ctx: &mut Context) -> Result<expr::Node> {
    let mut lhs = parse_postfix(ctx)?;
    loop {
        let loc = ctx.cur_loc();

        let star = ctx.skip_punct(PunctKind::Star);
        let slash = ctx.skip_punct(PunctKind::Slash);

        if !star && !slash {
            break;
        }

        let loc = loc?;
        let rhs = parse_postfix(ctx)?;

        lhs = expr::Node::new(
            expr::Kind::BinOp(
                if star {
                    expr::BinOpKind::Mul
                } else {
                    expr::BinOpKind::Div
                },
                Box::new(lhs.clone()),
                Box::new(rhs),
            ),
            loc,
        );
    }
    Ok(lhs)
}

pub fn parse_postfix(ctx: &mut Context) -> Result<expr::Node> {
    let base = parse_primary(ctx)?;
    let peek = match ctx.peek() {
        Some(peek) => peek,
        None => return Ok(base),
    };
    let loc = *peek.loc();
    match peek.kind() {
        // Call
        TokenKind::OpenDelim(DelimKind::Paren) => {
            assert!(ctx.next().is_some());
            ctx.expect_close_delim(DelimKind::Paren)?;
            Ok(expr::Node::new(
                expr::Kind::Call(Box::new(base), vec![]),
                loc,
            ))
        }
        _ => Ok(base),
    }
}

pub fn parse_primary(ctx: &mut Context) -> Result<expr::Node> {
    let peek = ctx.peek().ok_or(Error::EOF)?;
    let loc = *peek.loc();
    let node = match peek.kind() {
        TokenKind::Int(int) => Ok(expr::Node::new(expr::Kind::Int(int.parse().unwrap()), loc)),
        TokenKind::Ident(ident) if ident == &"func" => Ok(expr::Node::new(
            expr::Kind::Function(Box::new(function::parse(ctx)?)),
            loc,
        )),
        TokenKind::Ident(ident) => Ok(expr::Node::new(expr::Kind::Ident(ident.to_string()), loc)),
        _ => return Err(Error::ExpectedAny(loc, "integer value or identifier").into()),
    };
    ctx.next().unwrap();
    node
}

#[cfg(test)]
mod test {
    extern crate insta;
    use super::*;
    use crate::lexer::{source::Source, tokenize};

    #[test]
    fn parse1() {
        let source = Source::String(r#"x"#.to_string());
        let mut ctx = Context::new(tokenize(&source));
        insta::assert_debug_snapshot!(parse(&mut ctx).expect("fail to parse"));
    }

    #[test]
    fn parse2() {
        let source = Source::String(r#"x +x"#.to_string());
        let mut ctx = Context::new(tokenize(&source));
        insta::assert_debug_snapshot!(parse(&mut ctx).expect("fail to parse"));
    }

    #[test]
    fn parse3() {
        let source = Source::String(r#"123 + x"#.to_string());
        let mut ctx = Context::new(tokenize(&source));
        insta::assert_debug_snapshot!(parse(&mut ctx).expect("fail to parse"));
    }

    #[test]
    fn parse4() {
        let source = Source::String(r#"1 * 2 + 3"#.to_string());
        let mut ctx = Context::new(tokenize(&source));
        insta::assert_debug_snapshot!(parse(&mut ctx).expect("fail to parse"));
    }

    #[test]
    fn parse5() {
        let source = Source::String(r#"f()"#.to_string());
        let mut ctx = Context::new(tokenize(&source));
        insta::assert_debug_snapshot!(parse(&mut ctx).expect("fail to parse"));
    }
}
