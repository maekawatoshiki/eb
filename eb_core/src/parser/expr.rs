use super::Context;
use crate::{ast::expr, lexer::token::PunctKind};
use anyhow::Result;

pub fn parse(ctx: &mut Context) -> Result<expr::Node> {
    parse_binop_add_sub(ctx)
}

pub fn parse_binop_add_sub(ctx: &mut Context) -> Result<expr::Node> {
    let mut lhs = parse_primary(ctx)?;
    loop {
        let loc = ctx.cur_loc();

        let plus = ctx.skip_punct(PunctKind::Plus);
        let minus = ctx.skip_punct(PunctKind::Minus);

        if !plus && !minus {
            break;
        }

        let loc = loc?;
        let rhs = parse_primary(ctx)?;

        if plus {
            lhs = expr::Node::new(
                expr::Kind::BinOp(expr::BinOpKind::Plus, Box::new(lhs.clone()), Box::new(rhs)),
                loc,
            );
            continue;
        }

        if minus {
            lhs = expr::Node::new(
                expr::Kind::BinOp(expr::BinOpKind::Minus, Box::new(lhs.clone()), Box::new(rhs)),
                loc,
            );
            continue;
        }
    }
    Ok(lhs)
}

pub fn parse_primary(ctx: &mut Context) -> Result<expr::Node> {
    let loc = ctx.cur_loc()?;
    let ident = ctx
        .expect_any_ident()?
        .kind()
        .as_ident()
        .unwrap()
        .to_string();
    Ok(expr::Node::new(expr::Kind::Ident(ident), loc))
}

#[test]
fn parse1() {
    use crate::lexer::{location::Location, source::Source, tokenize};

    let source = Source::String(r#"x"#.to_string());
    let mut ctx = Context::new(tokenize(&source));
    assert_eq!(
        parse(&mut ctx).expect("fail to parse"),
        expr::Node::new(expr::Kind::Ident("x".to_string()), Location(0))
    );
}

#[test]
fn parse2() {
    use crate::lexer::{location::Location, source::Source, tokenize};

    let source = Source::String(r#"x +x"#.to_string());
    let mut ctx = Context::new(tokenize(&source));
    assert_eq!(
        parse(&mut ctx).expect("fail to parse"),
        expr::Node::new(
            expr::Kind::BinOp(
                expr::BinOpKind::Plus,
                Box::new(expr::Node::new(
                    expr::Kind::Ident("x".to_string()),
                    Location(0)
                )),
                Box::new(expr::Node::new(
                    expr::Kind::Ident("x".to_string()),
                    Location(3)
                ))
            ),
            Location(2)
        )
    );
}
