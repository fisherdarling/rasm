use crate::types::WasmResult;
use crate::types::WasmValue;

use crate::runtime::Runtime;

#[derive(Debug, Clone)]
pub enum StackElem {
    Value(WasmValue),
}

#[derive(Debug)]
pub struct Frame {
    pub(crate) locals: Vec<WasmValue>,
}

impl Frame {
    pub fn new(locals: Vec<WasmValue>) -> Self {
        Self { locals }
    }
}
