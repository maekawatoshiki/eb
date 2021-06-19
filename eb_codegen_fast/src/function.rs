use super::expr;
use anyhow::Result;
use ast::function as func;
use vm_ctx::FunctionContext as Context;

pub fn visit(ctx: &mut Context, func: &func::Node) -> Result<()> {
    ctx.name = func.name().to_owned();
    ctx.param_names = func.params().iter().map(|p| p.name().to_owned()).collect();
    expr::visit(ctx, func.body())?;
    Ok(())
}

#[cfg(test)]
mod test {
    extern crate eb_lexer as lexer;
    extern crate eb_parser as parser;
    extern crate insta;
    use super::*;
    use lexer::{source::Source, tokenize};
    use parser::{function::parse, Context as ParserContext};

    #[test]
    fn codegen1() {
        let source = Source::String(r#"func f(): ;;"#.to_string());
        let mut ctx = ParserContext::new(tokenize(&source));
        let node = parse(&mut ctx).expect("fail to parse");
        let mut ctx = Context::default();
        visit(&mut ctx, &node).unwrap();
        insta::assert_debug_snapshot!(ctx);
    }

    #[test]
    fn codegen2() {
        let source = Source::String(r#"func f(x): x;;"#.to_string());
        let mut ctx = ParserContext::new(tokenize(&source));
        let node = parse(&mut ctx).expect("fail to parse");
        let mut ctx = Context::default();
        visit(&mut ctx, &node).unwrap();
        insta::assert_debug_snapshot!(ctx);
    }

    #[test]
    fn codegen3() {
        let source = Source::String(
            r#"
            func f(x): 
                if x == 1:
                    return 1 ;;
                x * f(x - 1) ;;"#
                .to_string(),
        );
        let mut ctx = ParserContext::new(tokenize(&source));
        let node = parse(&mut ctx).expect("fail to parse");
        let mut ctx = Context::default();
        visit(&mut ctx, &node).unwrap();
        insta::assert_debug_snapshot!(ctx);
    }
}
