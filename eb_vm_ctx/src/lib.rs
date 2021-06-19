pub mod inst;
pub mod value;

#[derive(Debug, Clone)]
pub struct FunctionContext {
    pub name: String,
    pub param_names: Vec<String>,
    pub code: inst::Code,
    pub children: Vec<Self>, // TODO: Vec<Rc<Self>>
}

impl Default for FunctionContext {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            param_names: vec![],
            code: inst::Code(vec![]),
            children: vec![],
        }
    }
}

impl FunctionContext {
    pub fn push(&mut self, inst: inst::Inst) {
        self.code.0.push(inst)
    }

    pub fn add_child(&mut self, ctx: Self) {
        self.children.push(ctx)
    }
}
