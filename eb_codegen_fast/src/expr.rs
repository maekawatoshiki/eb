use super::function::{self, Context};
use anyhow::Result;
use ast::expr as ast_expr;
use vm::inst::Inst;

pub fn visit(ctx: &mut Context, expr: &ast_expr::Node) -> Result<()> {
    match expr.kind() {
        ast_expr::Kind::Int(i) => {
            ctx.push(Inst::PushInt(*i as i32)); // TODO
        }
        ast_expr::Kind::Ident(ident) => {
            ctx.push(Inst::PushStr(ident.to_owned()));
        }
        ast_expr::Kind::BinOp(op, lhs, rhs) => visit_binop(ctx, op, lhs, rhs)?,
        ast_expr::Kind::Function(func) => {
            let mut ctx_ = Context::default();
            function::visit(&mut ctx_, func)?;
            ctx.add_child(ctx_);
        }
        ast_expr::Kind::Call(callee, args) => visit_call(ctx, callee, args)?,
        ast_expr::Kind::If(cond, then_, else_) => visit_if(ctx, cond, then_, else_)?,
        ast_expr::Kind::Return(val) => visit_ret(ctx, val)?,
        ast_expr::Kind::Exprs(exprs) => {
            for expr in exprs {
                visit(ctx, expr)?;
            }
        }
    }
    Ok(())
}

fn visit_binop(
    ctx: &mut Context,
    op: &ast_expr::BinOpKind,
    lhs: &ast_expr::Node,
    rhs: &ast_expr::Node,
) -> Result<()> {
    visit(ctx, lhs)?;
    visit(ctx, rhs)?;
    match op {
        ast_expr::BinOpKind::Sub => ctx.push(Inst::Sub),
        ast_expr::BinOpKind::Mul => ctx.push(Inst::Mul),
        ast_expr::BinOpKind::Eq => ctx.push(Inst::Eq),
        _ => todo!(),
    }
    Ok(())
}

fn visit_call(ctx: &mut Context, callee: &ast_expr::Node, args: &[ast_expr::Node]) -> Result<()> {
    for arg in args {
        visit(ctx, arg)?;
    }
    match callee.kind() {
        ast_expr::Kind::Ident(name) => {
            ctx.push(Inst::PushStr(name.to_owned()));
        }
        _ => todo!(),
    }
    ctx.push(Inst::Call);
    Ok(())
}

fn visit_if(
    ctx: &mut Context,
    cond: &ast_expr::Node,
    then_: &ast_expr::Node,
    else_: &Option<Box<ast_expr::Node>>,
) -> Result<()> {
    assert!(else_.is_none());
    visit(ctx, cond)?;
    let cur = ctx.code.len() as i32;
    ctx.push(Inst::Jne(0));
    visit(ctx, then_)?;
    let merge = ctx.code.len() as i32;
    match ctx.code.get_mut(cur as usize).unwrap() {
        Inst::Jne(ref mut offset) => {
            *offset = merge - cur;
        }
        _ => panic!(),
    }
    Ok(())
}

fn visit_ret(ctx: &mut Context, val: &ast_expr::Node) -> Result<()> {
    visit(ctx, val)?;
    ctx.push(Inst::Ret);
    Ok(())
}
