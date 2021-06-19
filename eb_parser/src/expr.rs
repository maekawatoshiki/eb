use super::{function, Context, Error};
use crate::{
    ast::expr,
    lexer::token::{DelimKind, PunctKind, TokenKind},
};
use anyhow::Result;

pub fn parse(ctx: &mut Context) -> Result<expr::Node> {
    parse_binop_eq_ne(ctx)
}

fn parse_binop_eq_ne(ctx: &mut Context) -> Result<expr::Node> {
    let mut lhs = parse_binop_add_sub(ctx)?;
    loop {
        let loc = ctx.cur_loc();

        let eq = ctx.skip_punct(PunctKind::Eq);
        let neq = ctx.skip_punct(PunctKind::Neq);

        if !eq && !neq {
            break;
        }

        let loc = loc?;
        let rhs = parse_binop_add_sub(ctx)?;

        lhs = expr::Node::new(
            expr::Kind::BinOp(
                if eq {
                    expr::BinOpKind::Eq
                } else {
                    expr::BinOpKind::Neq
                },
                Box::new(lhs.clone()),
                Box::new(rhs),
            ),
            loc,
        );
    }
    Ok(lhs)
}

fn parse_binop_add_sub(ctx: &mut Context) -> Result<expr::Node> {
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

fn parse_binop_mul_div(ctx: &mut Context) -> Result<expr::Node> {
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

fn parse_postfix(ctx: &mut Context) -> Result<expr::Node> {
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
            Ok(expr::Node::new(
                expr::Kind::Call(Box::new(base), parse_call_args(ctx)?),
                loc,
            ))
        }
        _ => Ok(base),
    }
}

fn parse_call_args(ctx: &mut Context) -> Result<Vec<expr::Node>> {
    if ctx.skip_close_delim(DelimKind::Paren) {
        return Ok(vec![]);
    }

    let mut args = vec![];

    loop {
        let arg = parse(ctx)?;
        args.push(arg);

        if ctx.skip_punct(PunctKind::Comma) {
            continue;
        }

        ctx.expect_close_delim(DelimKind::Paren)?;

        break;
    }

    Ok(args)
}

fn parse_primary(ctx: &mut Context) -> Result<expr::Node> {
    let peek = ctx.peek().ok_or(Error::EOF)?;
    let loc = *peek.loc();
    match peek.kind() {
        TokenKind::Int(int) => {
            let int = int.parse().unwrap();
            ctx.next().unwrap();
            Ok(expr::Node::new(expr::Kind::Int(int), loc))
        }
        TokenKind::Ident(ident) if ident == &"func" => Ok(expr::Node::new(
            expr::Kind::Function(Box::new(function::parse(ctx)?)),
            loc,
        )),
        TokenKind::Ident(ident) if ident == &"if" => Ok(expr::Node::new(parse_if(ctx)?, loc)),
        TokenKind::Ident(ident) => {
            let ident = ident.to_string();
            ctx.next().unwrap();
            Ok(expr::Node::new(expr::Kind::Ident(ident), loc))
        }
        _ => Err(Error::ExpectedAny(loc, "integer value or identifier").into()),
    }
}

fn parse_if(ctx: &mut Context) -> Result<expr::Kind> {
    ctx.expect_keyword("if")?;
    let cond = parse(ctx)?;
    ctx.expect_punct(PunctKind::Colon)?;
    let loc = ctx.cur_loc()?;
    let then_expr = expr::Node::new(expr::Kind::Exprs(parse_body(ctx)?), loc);
    let else_expr;
    if ctx.skip_keyword("else") {
        ctx.expect_punct(PunctKind::Colon)?;
        let loc = ctx.cur_loc()?;
        else_expr = Some(Box::new(expr::Node::new(
            expr::Kind::Exprs(parse_body(ctx)?),
            loc,
        )));
    } else {
        else_expr = None;
    }
    Ok(expr::Kind::If(
        Box::new(cond),
        Box::new(then_expr),
        else_expr,
    ))
}

pub fn parse_body(ctx: &mut Context) -> Result<Vec<expr::Node>> {
    if ctx.skip_punct(PunctKind::DoubleSemicolon) {
        return Ok(vec![]);
    }

    let mut body = vec![];

    loop {
        body.push(parse(ctx)?);

        if ctx.skip_punct(PunctKind::Semicolon) {
            continue;
        }

        if ctx.skip_punct(PunctKind::DoubleSemicolon) {
            return Ok(body);
        }
    }
}

#[cfg(test)]
mod test {
    extern crate insta;
    use super::*;
    use crate::lexer::{source::Source, tokenize};

    fn parse_str(s: &str) -> expr::Node {
        let source = Source::String(s.to_string());
        let mut ctx = Context::new(tokenize(&source));
        parse(&mut ctx).expect("fail to parse")
    }

    #[test]
    fn parse1() {
        insta::assert_debug_snapshot!(parse_str(r#"x"#));
    }

    #[test]
    fn parse2() {
        insta::assert_debug_snapshot!(parse_str(r#"x +x"#));
    }

    #[test]
    fn parse3() {
        insta::assert_debug_snapshot!(parse_str(r#"123 + x"#));
    }

    #[test]
    fn parse4() {
        insta::assert_debug_snapshot!(parse_str(r#"1 * 2 + 3"#));
    }

    #[test]
    fn parse5() {
        insta::assert_debug_snapshot!(parse_str(r#"f()"#));
    }

    #[test]
    fn parse6() {
        insta::assert_debug_snapshot!(parse_str(r#"f(1, x)"#));
    }

    #[test]
    fn parse7() {
        insta::assert_debug_snapshot!(parse_str(r#"x == x"#));
    }

    #[test]
    fn parse8() {
        insta::assert_debug_snapshot!(parse_str(r#"x != x"#));
    }

    #[test]
    fn parse9() {
        insta::assert_debug_snapshot!(parse_str(
            r#"if x == 1:
                123;;"#
        ));
    }

    #[test]
    fn parse10() {
        insta::assert_debug_snapshot!(parse_str(
            r#"if x == 1:
                 x + 1 ; 
                 x ;;
               else:
                 42;;"#
        ));
    }
}
