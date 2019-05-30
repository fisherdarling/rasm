use std::collections::HashMap;
// use std::ops::Deref;
// use std::ops::DerefMut;

use crate::error::{ExecResult, Error};

use crate::runtime::frame::Frame;
use crate::function::{FuncRef, FuncInstance, FuncReader, Function};
use crate::types::{index::FuncIdx, Value, WasmResult};
use crate::instr::{Instr, Expression};
use crate::binop;

#[derive(Debug, Clone, Default)]
pub struct Interpreter {
    values: ValueStack,
    frames: FrameStack,
    functions: HashMap<FuncIdx, Function>,
    resolver: HashMap<String, FuncIdx>,
}

impl Interpreter {
    pub fn new(functions: HashMap<FuncIdx, Function>, resolver: HashMap<String, FuncIdx>) -> Interpreter {
        Interpreter {
            functions,
            resolver,
            ..Default::default()
        }
    }

    pub fn invoke<N: Into<String>, A: AsRef<[Value]>>(&mut self, name: N, args: A) -> ExecResult<WasmResult> {
        let name: String = name.into();
        let args: &[Value] = args.as_ref();

        let func_idx = self.resolver.get(&name).ok_or(Error::InvalidFunctionName(name))?;
        let function: &Function = self.functions.get(func_idx).ok_or(Error::InvalidFuncIdx(*func_idx))?;

        let frame = function.instantiate(args);

        self.execute_with_frame(frame)
    }

    fn execute_with_frame(&mut self, frame: Frame) -> ExecResult<WasmResult> {
        self.push_frame(frame);

        self.execute()
    }

    fn execute(&mut self) -> ExecResult<WasmResult> {
        let current_frame: &mut Frame = self.frames.inner().last_mut().ok_or(Error::EmptyFrameStack)?;

        loop {
            let next_instr = current_frame.reader().next().expect("Next instruction must not be none").clone();

            match next_instr {
                Instr::End => {
                    // Handle return of function
                },
                Instr::I32Const(c) => {
                    current_frame.stack.push(Value::I32(c));
                }
                Instr::I32Add => {
                    // self.call_add();
                }
                instr => return Err(Error::NotImplemented(instr.clone()))
            }


        }
    }

    fn push_frame(&mut self, frame: Frame) {
        self.frames.inner().push(frame);
    }

    fn pop_frame(&mut self) -> Option<Frame> {
        self.frames.inner().pop()
    }

}

#[derive(Debug, Default, Clone)]
pub struct FrameStack {
    frames: Vec<Frame>
}

impl FrameStack {
    pub fn inner(&mut self) -> &mut Vec<Frame> {
        &mut self.frames
    }
}


#[derive(Debug, Default, Clone, PartialEq)]
pub struct ValueStack {
    values: Vec<Value>,
}

impl ValueStack {
    pub fn with_capacity(cap: usize) -> ValueStack {
        ValueStack {
            values: Vec::with_capacity(cap)
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
        let rhs = self.values.pop().ok_or(Error::ValueStack)?;
        let lhs = self.values.pop().ok_or(Error::ValueStack)?;

        Ok((lhs, rhs))
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}