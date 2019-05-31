use crate::types::Value;
use crate::types::WasmResult;

use crate::error::{Error, ExecResult};
use crate::runtime::Runtime;

use crate::function::{FuncReader, FuncRef};

use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone)]
pub enum StackElem {
    Value(Value),
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub(crate) locals: Vec<Value>,
    pub(crate) stack: ValueStack,
    pub(crate) func: FuncRef,
    pub(crate) reader: Option<FuncReader>,
    pub(crate) is_block: bool,
}

impl Frame {
    pub fn new(locals: Vec<Value>, func: FuncRef) -> Frame {
        Frame {
            locals,
            stack: ValueStack::with_capacity(256),
            func,
            reader: None,
            is_block: false,
        }
    }

    pub fn gen_block(&self, func: FuncRef) -> Frame {
        let mut block = Frame::new(self.locals.clone(), func);

        block.stack = self.stack().clone();
        block.is_block = true;

        block
    }

    pub fn locals(&self) -> &[Value] {
        &self.locals
    }

    pub fn stack(&self) -> &ValueStack {
        &self.stack
    }

    pub fn stack_mut(&mut self) -> &mut ValueStack {
        &mut self.stack
    }

    pub fn push(&mut self, value: Value) {
        self.stack_mut().push(value);
    }

    pub fn pop(&mut self) -> ExecResult<Value> {
        self.stack_mut().pop()
    }

    pub fn pop_pair(&mut self) -> ExecResult<(Value, Value)> {
        self.stack_mut().pop_pair()
    }

    pub fn pause(&mut self, reader: FuncReader) {
        self.reader = Some(reader);
    }

    pub fn unpause(&mut self) -> ExecResult<FuncReader> {
        self.reader.take().ok_or(Error::UnpauseFrame)
    }

    pub fn is_paused(&self) -> bool {
        self.reader.is_some()
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

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ValueStack {
    values: Vec<Value>,
}

impl ValueStack {
    pub fn with_capacity(cap: usize) -> ValueStack {
        ValueStack {
            values: Vec::with_capacity(cap),
        }
    }

    pub fn push(&mut self, val: Value) {
        self.values.push(val);
    }

    pub fn pop(&mut self) -> ExecResult<Value> {
        let val = self.values.pop().ok_or(Error::ValueStack)?;

        Ok(val)
    }

    pub fn pop_pair(&mut self) -> ExecResult<(Value, Value)> {
        let rhs = self.values.pop().ok_or(Error::EmptyValueStack)?;
        let lhs = self.values.pop().ok_or(Error::EmptyValueStack)?;

        Ok((lhs, rhs))
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}
