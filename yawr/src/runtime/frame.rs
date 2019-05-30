use crate::types::WasmResult;
use crate::types::Value;

use crate::runtime::Runtime;

use crate::function::{FuncRef, FuncReader};

use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone)]
pub enum StackElem {
    Value(Value),
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub(crate) locals: Vec<Value>,
    pub(crate) stack: Vec<Value>,
    pub(crate) func: FuncRef,
}

impl Frame {
    pub fn new(locals: Vec<Value>, func: FuncRef) -> Frame {
        Frame {
            locals,
            stack: Vec::with_capacity(256),
            func,
        }
    }

    pub fn locals(&self) -> &[Value] {
        &self.locals
    }

    pub fn reader(&self) -> FuncReader {
        self.func.reader()
    }
}

impl Index<usize> for Frame {
    type Output = Value;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.locals[idx]
    }
}

impl IndexMut<usize> for Frame {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.locals[idx]
    }
}
