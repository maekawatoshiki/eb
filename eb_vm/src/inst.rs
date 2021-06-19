#[derive(Debug, Clone)]
pub enum Inst {
    PushInt(i32),
    PushStr(String),
    Call,
    Sub,
    Mul,
    Eq,
    Jne(i32),
    Ret,
}

pub struct Code(pub Vec<Inst>);

impl Code {
    pub fn get(&mut self, idx: usize) -> Option<&Inst> {
        self.0.get(idx)
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut Inst> {
        self.0.get_mut(idx)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

// pub struct Inst(u32);
//
// pub struct Code(pub Vec<Inst>);
//
// const CALL: u8 = 0x00;
// const ADD: u8 = 0x01;
// const SUB: u8 = 0x02;
// const MUL: u8 = 0x03;
// const JEQ: u8 = 0x04;
// const RET: u8 = 0x05;
//
// impl Inst {
//     pub fn opcode(self) -> u8 {
//         (self.0 >> 24) as u8
//     }
//
//     pub fn new_call() -> Self {
//         Inst((CALL as u32) << 24)
//     }
//
//     pub fn new_add() -> Self {
//         Inst((ADD as u32) << 24)
//     }
//
//     pub fn new_sub() -> Self {
//         Inst((SUB as u32) << 24)
//     }
//
//     pub fn new_mul() -> Self {
//         Inst((MUL as u32) << 24)
//     }
//
//     pub fn new_jeq() -> Self {
//         Inst((MUL as u32) << 24)
//     }
//
//     pub fn new_ret() -> Self {
//         Inst((RET as u32) << 24)
//     }
// }
