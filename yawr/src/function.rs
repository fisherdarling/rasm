// use std::collections::HashMap;

use crate::module::Module;
use crate::types::Function as PFunction;
use crate::types::{Locals, ResType, ValType, Value};
use crate::runtime::frame::Frame;

use std::rc::Rc;
use std::cell::Cell;
use std::ops::{Deref, Index};

use wasamere::instr::*;

#[derive(Debug, Clone)]
pub struct FuncRef(Rc<FuncInstance>);

impl FuncRef {
    pub fn new(func: FuncInstance) -> Self {
        FuncRef(Rc::new(func))
    }

    pub fn reader(&self) -> FuncReader {
        FuncReader::new(self.clone())
    }
}

impl Deref for FuncRef {
    type Target = FuncInstance;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}


#[derive(Debug, Clone)]
pub struct FuncInstance {
    res: ResType,
    body: Expression,
}

impl FuncInstance {
    pub fn new(res: ResType, body: Expression) -> FuncInstance {
        FuncInstance {
            res,
            body,
        }
    }

    pub fn len(&self) -> usize {
        self.body.len()
    }
}

impl Index<usize> for FuncInstance {
    type Output = Instr;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.body[idx]
    }
}

#[derive(Debug, Clone)]
pub struct FuncReader {
    instance: FuncRef,
    pos: Option<Cell<usize>>,
    finished: Cell<bool>,
}

impl FuncReader {
    pub fn new(func: FuncRef) -> FuncReader {
        FuncReader {
            instance: func,
            pos: None,
            finished: Cell::new(false),
        }
    }

    fn inc(&self) {
        match &self.pos {
            Some(pos) => { pos.set(pos.get() + 1); },
            None => {},
        }
    }

    fn dec(&self) {
        match &self.pos {
            Some(pos) => { pos.set(pos.get() - 1); },
            None => {},
        }
    }

    pub fn pos(&self) -> Option<usize> {
        self.pos.clone().map(|c: Cell<usize>| c.get())
    }

    pub fn current(&self) -> Option<&Instr> {
        if self.len() == 0 {
            return None
        }
        
        let instr = &self.instance[self.pos()?];

        Some(instr)
    }

    pub fn len(&self) -> usize {
        self.instance.len()
    }

    pub fn next(&self) -> Option<&Instr> {
        if self.pos()? + 1 >= self.len() {
            self.finished.replace(true);

            return None;
        }
        
        self.inc();

        self.current()
    }

    pub fn prev(&self) -> Option<&Instr> {
        if self.pos()? == 0 {
            if self.len() > 0 {
                self.finished.replace(false);
            }

            return None;
        }

        self.dec();

        self.current()
    }

    pub fn goto(&mut self, loc: usize) -> Option<&Instr> {
        if loc >= self.len() {
            return None;
        }

        match &self.pos {
            Some(pos) => pos.set(loc),
            None => panic!("Cannot goto without first executing next."),
        }

        self.current()
    }

    pub fn finished(&self) -> bool {
        self.finished.get()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub signature: Signature,
    pub locals: Locals,
    pub body: Expression,
}

impl Function {
    pub fn new(signature: Signature, locals: Locals, body: Expression) -> Self {
        Self {
            signature,
            locals,
            body,
        }
    }

    pub fn instantiate(&self, args: &[Value]) -> Frame {
        let mut locals: Vec<Value> = Vec::new();

        for (arg, param) in args.into_iter().zip(self.signature.params.iter()) {
            match (arg, param) {
                (a @ Value::I32(_), ValType::I32) => locals.push(a.clone()),
                (a @ Value::I64(_), ValType::I64) => locals.push(a.clone()),
                (a @ Value::F32(_), ValType::F32) => locals.push(a.clone()),
                (a @ Value::F64(_), ValType::F64) => locals.push(a.clone()),
                _ => panic!("Invalid argument types"),
            }
        }

        for local in &self.locals.0 {
            match local {
                ValType::I32 => locals.push(Value::I32(0)),
                ValType::I64 => locals.push(Value::I64(0)),
                ValType::F32 => locals.push(Value::F32(0.0)),
                ValType::F64 => locals.push(Value::F64(0.0)),
            }
        }

        let func = FuncRef::new(FuncInstance::new(self.signature.result, self.body.clone()));
        
        let frame = Frame::new(locals, func);

        frame
    } 
}

#[derive(Debug, Clone, PartialEq)]
pub struct Signature {
    pub(crate) params: Vec<ValType>,
    pub(crate) result: ResType,
}

impl From<crate::types::FuncType> for Signature {
    fn from(func: crate::types::FuncType) -> Self {
        Self {
            params: func.0,
            result: func.1,
        }
    }
}