use crate::runtime::interpreter::Interpreter;

pub static PAGE_SIZE: usize = 1 << 16;

#[derive(Debug, Clone, PartialEq)]
pub struct MemInst {
    data: Vec<u8>,
    max: Option<u32>,
}

impl MemInst {
    pub fn new(max: Option<u32>) -> Self {
        MemInst {
            data: Vec::new(),
            max,
        }
    }

    // pub fn 
}