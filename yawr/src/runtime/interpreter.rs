use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::{Deref, DerefMut};

use crate::error::{Error, ExecResult};

use crate::function::{FuncInstance, FuncReader, FuncRef, Function};
use crate::instr::{Expression, Instr};
use crate::runtime::frame::Frame;
use crate::types::{index::FuncIdx, ResType, ValType, Value, WasmResult};
use crate::{binop, is_a, relop, same_type};

use log::*;

#[derive(Debug, Clone, Default)]
pub struct Interpreter {
    frames: FrameStack,
    functions: HashMap<FuncIdx, Function>,
    resolver: HashMap<String, FuncIdx>,
}

impl Interpreter {
    pub fn new(
        functions: HashMap<FuncIdx, Function>,
        resolver: HashMap<String, FuncIdx>,
    ) -> Interpreter {
        Interpreter {
            functions,
            resolver,
            ..Default::default()
        }
    }

    pub fn invoke<N: Into<String>, A: AsRef<[Value]>>(
        &mut self,
        name: N,
        args: A,
    ) -> ExecResult<WasmResult> {
        let name: String = name.into();
        let args: &[Value] = args.as_ref();

        let func_idx = self
            .resolver
            .get(&name)
            .ok_or(Error::InvalidFunctionName(name))?;
        let function: &Function = self
            .functions
            .get(func_idx)
            .ok_or(Error::InvalidFuncIdx(*func_idx))?;

        let frame = function.instantiate(args)?;

        self.execute_with_frame(frame)
    }

    fn execute_with_frame(&mut self, frame: Frame) -> ExecResult<WasmResult> {
        self.push_frame(frame);

        self.execute()
    }

    fn execute(&mut self) -> ExecResult<WasmResult> {
        'frame: loop {
            let mut current_frame = self.pop_frame().ok_or(Error::EmptyFrameStack)?;
            let current_func = current_frame.func.clone();

            let mut reader = if current_frame.is_paused() {
                current_frame.unpause()?
            } else {
                current_func.reader()
            };

            'instr: loop {
                let next_instr = reader.next().expect("Next instruction must not be none");

                debug!("Next Instr: {:?}", next_instr);
                debug!("Value Stack: {:?}", current_frame.stack());

                match next_instr {
                    Instr::Drop => {
                        let _ = current_frame.pop()?;
                    }
                    Instr::Select => {
                        let value = is_a!(I32, current_frame.pop())?;

                        let val_2 = current_frame.pop()?;
                        let val_1 = current_frame.pop()?;

                        same_type!(val_1, val_2)?;

                        if bool::try_from(value)? {
                            current_frame.push(val_1);
                        } else {
                            current_frame.push(val_2);
                        }
                    }
                    Instr::End => {
                        debug!("[End], Reader: {:?}", reader.finished());

                        if !reader.finished() {
                            continue;
                        }

                        break 'instr;
                    }
                    Instr::LocalGet(idx) => {
                        let value = current_frame[idx.index() as usize];
                        current_frame.stack_mut().push(value);
                    }
                    Instr::LocalSet(idx) => {
                        let value = current_frame.pop()?;
                        current_frame[idx.index() as usize] = value;
                    }
                    Instr::LocalTee(idx) => {
                        let value = current_frame.pop()?;

                        current_frame.push(value.clone());

                        current_frame[idx.index() as usize] = value;
                    }
                    Instr::I32Const(c) => {
                        current_frame.push(Value::I32(*c));
                    }
                    Instr::I64Const(c) => {
                        current_frame.push(Value::I64(*c));
                    }
                    Instr::F32Const(c) => {
                        current_frame.push(Value::F32(*c));
                    }
                    Instr::F64Const(c) => {
                        current_frame.push(Value::F64(*c));
                    }
                    Instr::I32Add => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = binop!(I32, |a, b| a + b)(lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I32Eqz => {
                        let val = is_a!(I32, current_frame.pop())?;

                        let res = Value::from(val == Value::I32(0));
                        current_frame.push(res);
                    }
                    Instr::I32Eq => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = relop!(I32, |a, b| a == b)(lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I32Ne => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = relop!(I32, |a, b| a != b)(lhs, rhs)?;
                        current_frame.push(res);
                    }
                    instr => return Err(Error::NotImplemented(instr.clone())),
                }
            }

            if self.frames.len() == 0 {
                let result = match current_frame.func.res().clone() {
                    ResType::Unit => Ok(WasmResult::Unit),
                    ResType::ValType(v) => match (v, current_frame.pop()?) {
                        (ValType::I32, r @ Value::I32(_)) => Ok(WasmResult::from(r)),
                        (ValType::I64, r @ Value::I64(_)) => Ok(WasmResult::from(r)),
                        (ValType::F32, r @ Value::F32(_)) => Ok(WasmResult::from(r)),
                        (ValType::F64, r @ Value::F64(_)) => Ok(WasmResult::from(r)),
                        _ => Err(Error::TypeMismatch),
                    },
                };

                debug!("Returning to outer function: {:?}", result);

                return result;
            } else {
                let last_frame = self.peek_frame_mut().ok_or(Error::EmptyFrameStack)?;

                match current_frame.func.res().clone() {
                    ResType::Unit => {}
                    ResType::ValType(v) => {
                        let value = match (v, current_frame.pop()?) {
                            (ValType::I32, r @ Value::I32(_)) => Ok(r),
                            (ValType::I64, r @ Value::I64(_)) => Ok(r),
                            (ValType::F32, r @ Value::F32(_)) => Ok(r),
                            (ValType::F64, r @ Value::F64(_)) => Ok(r),
                            _ => Err(Error::TypeMismatch),
                        }?;

                        last_frame.push(value);
                    }
                };
            }
        }
    }

    fn push_frame(&mut self, frame: Frame) {
        self.frames.inner_mut().push(frame);
    }

    fn pop_frame(&mut self) -> Option<Frame> {
        self.frames.inner_mut().pop()
    }

    fn peek_frame_mut(&mut self) -> Option<&mut Frame> {
        self.frames.inner_mut().last_mut()
    }
}

#[derive(Debug, Default, Clone)]
pub struct FrameStack {
    frames: Vec<Frame>,
}

impl FrameStack {
    pub fn inner(&self) -> &[Frame] {
        &self.frames
    }

    pub fn inner_mut(&mut self) -> &mut Vec<Frame> {
        &mut self.frames
    }

    pub fn len(&self) -> usize {
        self.frames.len()
    }
}
