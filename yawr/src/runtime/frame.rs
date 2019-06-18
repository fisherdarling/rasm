use crate::types::Value;
use crate::types::{ResType, WasmResult};

use crate::error::{Error, ExecResult};
use crate::runtime::Runtime;

use crate::function::{FuncReader, FuncRef};

use std::fmt;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone)]
pub enum StackElem {
    Value(Value),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LabelType {
    Block(ResType, usize),
    If(ResType, usize, usize),
    Loop(ResType, usize),
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub locals: Vec<Value>,
    pub stack: ValueStack,
    pub label_stack: Vec<LabelType>,
    pub func: FuncRef,
    pub reader: Option<FuncReader>,
}

impl Frame {
    pub fn new(locals: Vec<Value>, func: FuncRef) -> Frame {
        Frame {
            locals,
            stack: ValueStack::with_capacity(10),
            label_stack: Vec::new(),
            func,
            reader: None,
        }
    }

    pub fn res(&self) -> ResType {
        self.func.res()
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

#[derive(Default, Clone, PartialEq)]
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
        let val = self.values.pop().ok_or(Error::EmptyValueStack)?;

        Ok(val)
    }

    pub fn peek(&self) -> Option<&Value> {
        self.values.last()
    }

    pub fn pop_pair(&mut self) -> ExecResult<(Value, Value)> {
        let rhs = self.values.pop().ok_or(Error::EmptyValueStack)?;
        let lhs = self.values.pop().ok_or(Error::EmptyValueStack)?;

        Ok((lhs, rhs))
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn pop_args(&mut self, count: usize) -> ExecResult<Vec<Value>> {
        let mut args: Vec<Value> = vec![Value::I32(0); count];

        for i in 0..count {
            args[count - i - 1] = self.pop()?;
        }

        Ok(args)
    }
}

impl fmt::Debug for ValueStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.values)
    }
}
