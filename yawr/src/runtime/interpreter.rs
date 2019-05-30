use std::collections::HashMap;
// use std::ops::Deref;
// use std::ops::DerefMut;

use crate::error::{ExecResult, Error};

use crate::runtime::frame::Frame;
use crate::function::{FuncRef, FuncInstance, FuncReader, Function};
use crate::types::{index::FuncIdx, Value, WasmResult, ResType, ValType};
use crate::instr::{Instr, Expression};
use crate::binop;

use log::*;

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
        loop {
            let current_frames_len = self.frames.len();
            
            debug!("Number of Stack Frames: {:?}", current_frames_len);
            
            let current_frame: &mut Frame = self.frames.inner().last_mut().ok_or(Error::EmptyFrameStack)?;
            let mut reader = current_frame.reader();

            loop {
                let next_instr = reader.next().expect("Next instruction must not be none");

                debug!("Next Instr: {:?}", next_instr);
                debug!("Value Stack: {:?}", self.values);

                match next_instr {
                    Instr::LocalGet(idx) => {
                        let value = current_frame[idx.index() as usize];
                        self.values.push(value);
                    }
                    Instr::End => {
                        debug!("[End], Reader: {:?}", reader.finished());

                        if !reader.finished() {
                            continue;
                        }

                        break;

                        // match current_frame.func.res() {
                        //     ResType::Unit => {},
                        //     ResType::ValType(v) => {
                        //         match (v, self.values.pop()?) {
                        //             (ValType::I32, Value::I32(i)) => {},
                        //             _ => {},
                        //         }
                        //     },
                        // }
                    },
                    Instr::I32Const(c) => {
                        self.values.push(Value::I32(*c));
                        // current_frame.stack.push(Value::I32(c.clone()));
                    }
                    Instr::I32Add => {
                        let (lhs, rhs) = self.values.pop_pair()?;

                        let res = binop!(I32, |a, b| a + b)(lhs, rhs)?;
                        self.values.push(res);
                    }
                    instr => return Err(Error::NotImplemented(instr.clone()))
                }
            }

            if current_frames_len == 1 {

                // Handle Returning to the outer function:

                let result = match current_frame.func.res() {
                    ResType::Unit => Ok(WasmResult::Unit),
                    ResType::ValType(v) => {
                        match (v, self.values.pop()?) {
                            (ValType::I32, r @ Value::I32(_)) => Ok(WasmResult::from(r)),
                            (ValType::I64, r @ Value::I64(_)) => Ok(WasmResult::from(r)),
                            (ValType::F32, r @ Value::F32(_)) => Ok(WasmResult::from(r)),
                            (ValType::F64, r @ Value::F64(_)) => Ok(WasmResult::from(r)),
                            _ => Err(Error::TypeMismatch)
                        }
                    }
                };

                debug!("Returning to outer function: {:?}", result);

                return result;
            } else {
                self.pop_frame();
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

    pub fn len(&self) -> usize {
        self.frames.len()
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