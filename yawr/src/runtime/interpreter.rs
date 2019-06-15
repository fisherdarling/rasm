use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::{Deref, DerefMut};

use crate::error::{Error, ExecResult};

use crate::function::{FuncInstance, FuncReader, FuncRef, Function};
use crate::instr::{Expression, Instr};
use crate::runtime::frame::{Frame, LabelType};
use crate::types::{index::FuncIdx, ResType, ValType, Value, WasmResult};
use crate::{binop, is_a, relop, same_type, truthy, valid_result};
use crate::math::*;

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
                let next_instr = reader.next().expect("Next instruction must not be none").clone();

                debug!("Next Instr: {:?}", next_instr);
                // debug!("Value Stack: {:?}", current_frame.stack());

                match next_instr {
                    Instr::Drop => {
                        let _ = current_frame.pop()?;
                    }
                    Instr::IfMarker(result, true_end, false_end) => {
                        let check = current_frame.pop()?;

                        if truthy!(check)? {
                            // Do nothing, the next instruction is part of the truthy path.
                        } else {
                            // Skip the true path
                            reader.goto(true_end);
                        }

                        current_frame.label_stack.push(LabelType::If(result, true_end, false_end));
                    }
                    Instr::BlockMarker(result, block_end) => {
                        current_frame.label_stack.push(LabelType::Block(result, block_end));
                    }
                    Instr::LoopMarker(result, loop_start) => {
                        current_frame.label_stack.push(LabelType::Loop(result, loop_start));
                    }
                    Instr::Select => {
                        let check = current_frame.pop()?;
                        let val_2 = current_frame.pop()?;
                        let val_1 = current_frame.pop()?;

                        same_type!(val_1, val_2)?;

                        if truthy!(check)? {
                            current_frame.push(val_1);
                        } else {
                            current_frame.push(val_2);
                        }
                    }
                    Instr::Br(idx) => {
                        for i in 0..*idx {
                            current_frame.label_stack.pop().ok_or(Error::BranchDepth(idx))?;
                        }

                        if current_frame.label_stack.is_empty() {
                            break 'instr;
                        }

                        let target = current_frame.label_stack.pop().ok_or(Error::BranchDepth(idx))?;
                        debug!("Branch Target: {:?}", target);
                        match target {
                            LabelType::If(result, true_end, false_end) => {
                                if let Some(value) = current_frame.stack().peek() {
                                    valid_result!(result, value)?;
                                    reader.goto(false_end);
                                } else if result != ResType::Unit {
                                    return Err(Error::TypeMismatch);
                                }
                            },
                            LabelType::Block(result, block_end) => {
                                if let Some(value) = current_frame.stack().peek() {
                                    valid_result!(result, value)?;
                                    reader.goto(block_end);
                                } else if result != ResType::Unit {
                                    return Err(Error::TypeMismatch);
                                }
                            },
                            LabelType::Loop(result, loop_start) => {
                                if let Some(value) = current_frame.stack().peek() {
                                    valid_result!(result, value)?;
                                    reader.goto(loop_start);
                                    debug!("Reader Position: {:?}", reader.pos());
                                    current_frame.label_stack.push(LabelType::Loop(result, loop_start));
                                } else if result == ResType::Unit {
                                    reader.goto(loop_start);
                                    debug!("Reader Position: {:?}", reader.pos());
                                    current_frame.label_stack.push(LabelType::Loop(result, loop_start));
                                } else {
                                    return Err(Error::TypeMismatch);
                                }
                            }
                        }   


                    }
                    Instr::End => {
                        debug!("[End], Reader: {:?}, Scope: {:?}", reader.finished(), current_frame.label_stack);

                        if current_frame.label_stack.is_empty() {
                            // assert!(reader.finished());
                            break 'instr;
                        }

                        let outer_label = current_frame.label_stack.pop().expect("Label stack must contain a label"); 
                        match outer_label {
                            LabelType::If(result, true_end, false_end) => {
                                if let Some(value) = current_frame.stack().peek() {
                                    valid_result!(result, value)?;
                                    reader.goto(false_end);
                                } else if result != ResType::Unit {
                                    return Err(Error::TypeMismatch);
                                }
                            },
                            LabelType::Block(result, _block_end) => {
                                if let Some(value) = current_frame.stack().peek() {
                                    valid_result!(result, value)?;
                                } else if result != ResType::Unit {
                                    return Err(Error::TypeMismatch);
                                }
                            },
                            LabelType::Loop(result, _loop_end) => {
                                if let Some(value) = current_frame.stack().peek() {
                                    valid_result!(result, value)?;
                                } else if result != ResType::Unit {
                                    return Err(Error::TypeMismatch);
                                }
                            }
                        }                        
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
                        current_frame.push(Value::I32(c));
                    }
                    Instr::I64Const(c) => {
                        current_frame.push(Value::I64(c));
                    }
                    Instr::F32Const(c) => {
                        current_frame.push(Value::F32(c));
                    }
                    Instr::F64Const(c) => {
                        current_frame.push(Value::F64(c));
                    }
                    Instr::I32Add => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = binop!(I32, +)(lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I32Eqz => {
                        let val = is_a!(I32, current_frame.pop())?;

                        let res = Value::from(val == Value::I32(0));
                        current_frame.push(res);
                    }
                    Instr::I32Eq => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = relop!(I32, ==)(lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I32Ne => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = relop!(I32, !=)(lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I32GtS => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = relop!(I32, >, cast: i32)(lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I32GtU => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = relop!(I32, >)(lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I32LtS => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = relop!(I32, |a, b| (a as i32) < b as i32)(lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I32LtU => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = relop!(I32, |a, b| a < b)(lhs, rhs)?;
                        current_frame.push(res);
                    }
                    instr => return Err(Error::NotImplemented(instr.clone())),
                }

                debug!("Label Stack: {:?}", current_frame.label_stack);
            }

            if self.frames.len() == 0 {
                let result = match current_frame.func.res().clone() {
                    ResType::Unit => Ok(WasmResult::Unit),
                    res => match (res, current_frame.pop()?) {
                        (ResType::I32, r @ Value::I32(_)) => Ok(WasmResult::from(r)),
                        (ResType::I64, r @ Value::I64(_)) => Ok(WasmResult::from(r)),
                        (ResType::F32, r @ Value::F32(_)) => Ok(WasmResult::from(r)),
                        (ResType::F64, r @ Value::F64(_)) => Ok(WasmResult::from(r)),
                        _ => Err(Error::TypeMismatch),
                    },
                };

                debug!("Returning to outer function: {:?}", result);

                return result;
            } else {
                let last_frame = self.peek_frame_mut().ok_or(Error::EmptyFrameStack)?;

                match current_frame.func.res().clone() {
                    ResType::Unit => {}
                    res => {
                        let value = match (res, current_frame.pop()?) {
                            (ResType::I32, r @ Value::I32(_)) => Ok(r),
                            (ResType::I64, r @ Value::I64(_)) => Ok(r),
                            (ResType::F32, r @ Value::F32(_)) => Ok(r),
                            (ResType::F64, r @ Value::F64(_)) => Ok(r),
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
