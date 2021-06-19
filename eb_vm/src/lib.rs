extern crate eb_vm_ctx as vm_ctx;
extern crate rustc_hash;

use rustc_hash::FxHashMap;
use vm_ctx::value::Value;
use vm_ctx::FunctionContext;

pub struct VM {
    pub stack: Vec<Value>,
    pub env: Vec<FxHashMap<String, Value>>,
}

impl Default for VM {
    fn default() -> Self {
        Self {
            stack: vec![],
            env: vec![],
        }
    }
}

impl VM {
    pub fn run(&mut self, _ctx: &FunctionContext) {}
}

#[test]
fn vm1() {
    extern crate eb_codegen_fast as codegen;
    extern crate eb_lexer as lexer;
    extern crate eb_parser as parser;
    use codegen::expr::visit;
    use lexer::{source::Source, tokenize};
    use parser::{expr::parse_body, Context as ParserContext};

    let source = Source::String(
        r#"
            func f(x): 
                if x == 1:
                    return 1 ;;
                x * f(x - 1) ;;
            f(10) ;;"#
            .to_string(),
    );
    let mut ctx = ParserContext::new(tokenize(&source));
    let node = parse_body(&mut ctx).expect("fail to parse");
    let mut ctx_ = FunctionContext::default();
    visit(&mut ctx_, &node).unwrap();
    let mut vm = VM::default();
    vm.run(&ctx_);
}
