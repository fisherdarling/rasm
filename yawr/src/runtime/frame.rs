use crate::types::Value;
use crate::types::{ResType, WasmResult};

use crate::error::{Error, ExecResult};
use crate::runtime::ModuleInstance;

use crate::function::{FuncReader, FuncRef};

use std::fmt;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LabelType {
    Block(ResType, usize),
    If(ResType, usize, usize),
    Loop(ResType, usize),
}

impl LabelType {
    pub fn res(&self) -> ResType {
        match self {
            LabelType::Block(res, _) => *res,
            LabelType::If(res, _, _) => *res,
            LabelType::Loop(res, _) => *res, 
        }
    }
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
    values: Vec<StackElem>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StackElem {
    Value(Value),
    Label,
}

impl StackElem {
    pub fn into_value(self) -> ExecResult<Value> {
        match self {
            StackElem::Value(val) => Ok(val),
            _ => Err(Error::StackElemIntoValue),
        }
    }

    pub fn as_value(&self) -> ExecResult<&Value> {
        match &self {
            StackElem::Value(ref val) => Ok(val),
            _ => Err(Error::StackElemIntoValue),
        }
    }
}

impl ValueStack {
    // pub fn into_value()



    pub fn with_capacity(cap: usize) -> ValueStack {
        ValueStack {
            values: Vec::with_capacity(cap),
        }
    }

    pub fn push(&mut self, val: Value) {
        self.values.push(StackElem::Value(val));
    }

    pub fn push_label(&mut self) {
        self.values.push(StackElem::Label);
    }

    pub fn pop(&mut self) -> ExecResult<Value> {
        self.values.pop().ok_or(Error::EmptyValueStack)?.into_value()
    }

    pub fn pop_label_depth(&mut self, depth: Option<usize>) {
        if let Some(depth) = depth {
            for _ in 0..=depth {
                
                while let Some(StackElem::Value(ref v)) = self.values.last() {
                    log::debug!("[STACK CLEAN] Value: {:?}", v);
                    self.values.pop();
                }
                
                let label = self.values.pop();
                
                log::debug!("[STACK CLEAN] Label: {:?}", label);
            }
        }
        
        return;
    }

    pub fn peek_value(&self) -> ExecResult<Option<&Value>> {
        self.values.last().map(StackElem::as_value).transpose()
    }

    pub fn pop_pair(&mut self) -> ExecResult<(Value, Value)> {
        let rhs = self.pop()?;
        let lhs = self.pop()?;

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
