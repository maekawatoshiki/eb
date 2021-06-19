extern crate eb_vm_ctx as vm_ctx;
extern crate rustc_hash;

use rustc_hash::FxHashMap;
use vm_ctx::inst::Inst;
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
    fn lookup(&mut self, s: &str) -> Option<&Value> {
        for e in self.env.iter().rev() {
            if let Some(v) = e.get(s) {
                return Some(v);
            }
        }
        None
    }

    pub fn run(&mut self, ctx: &FunctionContext) {
        self.env.push({
            let mut map = FxHashMap::default();
            for child in &ctx.children {
                map.insert(child.name.clone(), Value::Func(Box::new(child.clone())));
            }
            map
        });

        let mut pc_stack = vec![0];
        let mut code_stack = vec![ctx.code.0.clone()];
        loop {
            if *pc_stack.last().unwrap() >= code_stack.last().unwrap().len() {
                self.env.pop().unwrap();
                code_stack.pop();
                pc_stack.pop();
                if self.env.len() == 1 {
                    break;
                }
            }
            let inst = &code_stack.last().unwrap()[*pc_stack.last().unwrap()];
            match inst {
                Inst::PushInt(i) => {
                    self.stack.push(Value::Int(*i));
                    *pc_stack.last_mut().unwrap() += 1;
                }
                Inst::PushStr(s) => {
                    self.stack.push(Value::String(s.clone()));
                    *pc_stack.last_mut().unwrap() += 1;
                }
                Inst::Get(s) => {
                    let val = self.lookup(s).unwrap().clone();
                    self.stack.push(val.clone());
                    *pc_stack.last_mut().unwrap() += 1;
                }
                Inst::Call => {
                    *pc_stack.last_mut().unwrap() += 1;
                    let callee = self.stack.pop().unwrap();
                    match callee {
                        Value::Func(func) => {
                            let mut args = vec![];
                            for _ in 0..func.param_names.len() {
                                args.push(self.stack.pop().unwrap());
                            }
                            self.env.push({
                                let mut map = FxHashMap::default();
                                for child in &func.children {
                                    map.insert(
                                        child.name.clone(),
                                        Value::Func(Box::new(child.clone())),
                                    );
                                }
                                for (param, val) in
                                    func.param_names.iter().zip(args.into_iter().rev())
                                {
                                    map.insert(param.clone(), val);
                                }
                                map
                            });
                            pc_stack.push(0);
                            code_stack.push(func.code.0.clone());
                            continue;
                        }
                        _ => todo!(),
                    }
                }
                Inst::Sub => {
                    let rhs = self.stack.pop().unwrap();
                    let lhs = self.stack.pop().unwrap();
                    match (lhs, rhs) {
                        (Value::Int(lhs), Value::Int(rhs)) => {
                            self.stack.push(Value::Int(lhs - rhs));
                        }
                        _ => todo!(),
                    }
                    *pc_stack.last_mut().unwrap() += 1;
                }
                Inst::Mul => {
                    let rhs = self.stack.pop().unwrap();
                    let lhs = self.stack.pop().unwrap();
                    match (lhs, rhs) {
                        (Value::Int(lhs), Value::Int(rhs)) => {
                            self.stack.push(Value::Int(lhs * rhs));
                        }
                        _ => todo!(),
                    }
                    *pc_stack.last_mut().unwrap() += 1;
                }
                Inst::Eq => {
                    let rhs = self.stack.pop().unwrap();
                    let lhs = self.stack.pop().unwrap();
                    match (lhs, rhs) {
                        (Value::Int(lhs), Value::Int(rhs)) => {
                            self.stack.push(Value::Bool(lhs == rhs));
                        }
                        _ => todo!(),
                    }
                    *pc_stack.last_mut().unwrap() += 1;
                }
                Inst::Jne(offset) => {
                    let val = self.stack.pop().unwrap();
                    match val {
                        Value::Bool(false) => {
                            *pc_stack.last_mut().unwrap() += *offset as usize;
                        }
                        _ => {
                            *pc_stack.last_mut().unwrap() += 1;
                        }
                    }
                }
                Inst::Ret => {
                    self.env.pop().unwrap();
                    code_stack.pop();
                    pc_stack.pop();
                    continue;
                }
            }
        }
    }
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
    assert!(matches!(vm.stack.pop().unwrap(), Value::Int(3628800)));
}
