use std::collections::HashMap;
use std::convert::{TryInto, TryFrom};
use std::ops::{Deref, DerefMut};

use crate::error::{Error, ExecResult};

use crate::function::{FuncInstance, FuncReader, FuncRef, Function};
use crate::instr::{Expression, Instr};
use crate::runtime::frame::{Frame, LabelType};
use crate::types::{index::FuncIdx, ResType, ValType, Value, WasmResult};
// use crate::{binop, is_a, relop, same_type, truthy, valid_result};
use crate::*;
use crate::math;

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

                debug!("[{:2?}] [Instr] {:?}", reader.pos().unwrap(), next_instr);
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
                                if result == ResType::Unit {
                                    reader.goto(false_end);
                                } else if let Some(value) = current_frame.stack().peek() {
                                    valid_result!(result, value)?;
                                    reader.goto(false_end);
                                } else {
                                    return Err(Error::TypeMismatch(line!()));
                                }
                            },
                            LabelType::Block(result, block_end) => {
                                if result == ResType::Unit {
                                    reader.goto(block_end);
                                } else if let Some(value) = current_frame.stack().peek() {
                                    valid_result!(result, value)?;
                                    reader.goto(block_end);
                                } else {
                                    return Err(Error::TypeMismatch(line!()));
                                }
                            },
                            LabelType::Loop(result, loop_start) => {
                                if result == ResType::Unit {
                                    reader.goto(loop_start);
                                    debug!("Reader Position: {:?}", reader.pos());
                                    current_frame.label_stack.push(LabelType::Loop(result, loop_start));
                                } else if let Some(value) = current_frame.stack().peek() {
                                    valid_result!(result, value)?;
                                    reader.goto(loop_start);
                                    debug!("Reader Position: {:?}", reader.pos());
                                    current_frame.label_stack.push(LabelType::Loop(result, loop_start));
                                } else {
                                    return Err(Error::TypeMismatch(line!()));
                                }
                            }
                        }   
                    }
                    Instr::BrIf(idx) => {
                        let check = current_frame.pop()?;

                        if !truthy!(check)? {
                            debug!("\t---> BrIf NOP");
                            continue;
                        }


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
                                    return Err(Error::TypeMismatch(line!()));
                                }
                            },
                            LabelType::Block(result, block_end) => {
                                if let Some(value) = current_frame.stack().peek() {
                                    valid_result!(result, value)?;
                                    reader.goto(block_end);
                                } else if result != ResType::Unit {
                                    return Err(Error::TypeMismatch(line!()));
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
                                    return Err(Error::TypeMismatch(line!()));
                                }
                            }
                        }   
                    }
                    Instr::Call(idx) => {
                        let function = self.functions.get(&idx).ok_or(Error::InvalidFuncIdx(idx.clone()))?;
                        
                        let num_values = function.argument_length();
                        let mut args: Vec<Value> = vec![Value::I32(0); num_values];

                        for i in 0..num_values {
                            args[num_values - i - 1] = current_frame.pop()?;
                        }

                        let new_frame = function.instantiate(&args)?;
                        
                        current_frame.pause(reader);
                        self.push_frame(current_frame);
                        self.push_frame(new_frame);

                        continue 'frame;
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
                                    return Err(Error::TypeMismatch(line!()));
                                }
                            },
                            LabelType::Block(result, _block_end) => {
                                if let Some(value) = current_frame.stack().peek() {
                                    valid_result!(result, value)?;
                                } else if result != ResType::Unit {
                                    return Err(Error::TypeMismatch(line!()));
                                }
                            },
                            LabelType::Loop(result, _loop_end) => {
                                if let Some(value) = current_frame.stack().peek() {
                                    valid_result!(result, value)?;
                                } else if result != ResType::Unit {
                                    return Err(Error::TypeMismatch(line!()));
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

                    // I32 RELOP
                    Instr::I32Const(c) => {
                        current_frame.push(Value::I32(c as i32));
                    }
                    Instr::I64Const(c) => {
                        current_frame.push(Value::I64(c as i64));
                    }
                    Instr::F32Const(c) => {
                        current_frame.push(Value::F32(c));
                    }
                    Instr::F64Const(c) => {
                        current_frame.push(Value::F64(c));
                    }
                    Instr::I32Eqz => {
                        let val = is_a!(I32, current_frame.pop())?;

                        let res = Value::from(val == Value::I32(0));
                        current_frame.push(res);
                    }
                    Instr::I32Eq => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = ieq!(I32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I32Ne => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = ine!(I32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I32GtS => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = igt_s!(I32, lhs, rhs, i32)?;
                        current_frame.push(res);
                    }
                    Instr::I32GtU => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = igt_u!(I32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I32LtS => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = ilt_s!(I32, lhs, rhs, i32)?;
                        current_frame.push(res);
                    }
                    Instr::I32LtU => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = ilt_u!(I32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I32LeU => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = ile_u!(I32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I32LeS => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = ile_s!(I32, lhs, rhs, i32)?;
                        current_frame.push(res);
                    }
                    Instr::I32GeU => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = ige_u!(I32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I32GeS => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = ige_s!(I32, lhs, rhs, i32)?;
                        current_frame.push(res);
                    }
                    
                    // I64 RELOP
                    Instr::I64Eqz => {
                        let val = is_a!(I64, current_frame.pop())?;

                        let res = Value::from(val == Value::I64(0));
                        current_frame.push(res);
                    }
                    Instr::I64Eq => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = ieq!(I64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I64Ne => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = ine!(I64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I64GtS => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = igt_s!(I64, lhs, rhs, i64)?;
                        current_frame.push(res);
                    }
                    Instr::I64GtU => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = igt_u!(I64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I64LtS => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = ilt_s!(I64, lhs, rhs, i64)?;
                        current_frame.push(res);
                    }
                    Instr::I64LtU => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = ilt_u!(I64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I64LeU => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = ile_u!(I64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I64LeS => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = ile_s!(I64, lhs, rhs, i64)?;
                        current_frame.push(res);
                    }
                    Instr::I64GeU => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = ige_u!(I64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I64GeS => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = ige_s!(I64, lhs, rhs, i64)?;
                        current_frame.push(res);
                    }

                    // F32 RELOP
                    Instr::F32Eq => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = feq!(F32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::F32Ne => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = fne!(F32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::F32Lt => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = flt!(F32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::F32Gt => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = fgt!(F32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::F32Le => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = fle!(F32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::F32Ge => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = fge!(F32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    
                    // F64 RELOP
                    Instr::F64Eq => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = feq!(F64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::F64Ne => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = fne!(F64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::F64Lt => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = flt!(F64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::F64Gt => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = fgt!(F64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::F64Le => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = fle!(F64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::F64Ge => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = fge!(F64, lhs, rhs)?;
                        current_frame.push(res);
                    }

                    // I32 BINOP / UNOP, TODO SHL/R(S/U) ROTL/ROTR
                    Instr::I32Clz => {
                        let value = current_frame.pop()?;

                        let res = iclz!(I32, value)?;
                        current_frame.push(res);
                    }
                    Instr::I32Ctz => {
                        let value = current_frame.pop()?;

                        let res = ictz!(I32, value)?;
                        current_frame.push(res);
                    }
                    Instr::I32Popcnt => {
                        let value = current_frame.pop()?;

                        let res = ipopcnt!(I32, value)?;
                        current_frame.push(res);
                    }
                    Instr::I32Add => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = iadd!(I32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I32Sub => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = isub!(I32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I32Mul => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = imul!(I32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I32DivS => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = idiv_s!(I32, lhs, rhs, i32)?;
                        current_frame.push(res);
                    }
                    Instr::I32DivU => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = idiv_u!(I32, lhs, rhs)?;
                        current_frame.push(res);
                    }

                    Instr::I32RemS => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = irem_s!(I32, lhs, rhs, i32)?;
                        current_frame.push(res);
                    }
                    Instr::I32RemU => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = irem_u!(I32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I32And => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = iand!(I32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I32Or => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = ior!(I32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I32Xor => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = ixor!(I32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I32Shl => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = shl!(I32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I32ShrS => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = shr!(I32, lhs, rhs, i32)?;
                        current_frame.push(res);
                    }
                    Instr::I32ShrU => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = shr!(I32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I32Rotl => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = rotl!(I32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I32Rotr => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = rotr!(I32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    

                    // I64 BINOP / UNOP TODO SHL/R(S/U) ROTL/ROTR
                    Instr::I64Clz => {
                        let value = current_frame.pop()?;

                        let res = iclz!(I64, value)?;
                        current_frame.push(res);
                    }
                    Instr::I64Ctz => {
                        let value = current_frame.pop()?;

                        let res = ictz!(I64, value)?;
                        current_frame.push(res);
                    }
                    Instr::I64Popcnt => {
                        let value = current_frame.pop()?;

                        let res = ipopcnt!(I64, value)?;
                        current_frame.push(res);
                    }
                    Instr::I64Add => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = iadd!(I64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I64Sub => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = isub!(I64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I64Mul => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = imul!(I64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I64DivS => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = idiv_s!(I64, lhs, rhs, i64)?;
                        current_frame.push(res);
                    }
                    Instr::I64DivU => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = idiv_u!(I64, lhs, rhs)?;
                        current_frame.push(res);
                    }

                    Instr::I64RemS => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = irem_s!(I64, lhs, rhs, i64)?;
                        current_frame.push(res);
                    }
                    Instr::I64RemU => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = irem_u!(I64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I64And => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = iand!(I64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I64Or => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = ior!(I64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I64Xor => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = ixor!(I64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I64Shl => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = shl!(I64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I64ShrS => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = shr!(I64, lhs, rhs, i32)?;
                        current_frame.push(res);
                    }
                    Instr::I64ShrU => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = shr!(I64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I64Rotl => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = rotl!(I64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I64Rotr => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = rotr!(I64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    
                    // F32 BINOP / UNOP
                    Instr::F32Abs => {
                        let value = current_frame.pop()?;

                        let res = fabs!(F32, value)?;
                        current_frame.push(res);
                    }
                    Instr::F32Neg => {
                        let value = current_frame.pop()?;

                        let res = fneg!(F32, value)?;
                        current_frame.push(res);
                    }
                    Instr::F32Ceil => {
                        let value = current_frame.pop()?;

                        let res = fceil!(F32, value)?;
                        current_frame.push(res);
                    }
                    Instr::F32Floor => {
                        let value = current_frame.pop()?;

                        let res = ffloor!(F32, value)?;
                        current_frame.push(res);
                    }
                    Instr::F32Trunc => {
                        let value = current_frame.pop()?;

                        let res = ftrunc!(F32, value)?;
                        current_frame.push(res);
                    }
                    Instr::F32Nearest => {
                        let value = current_frame.pop()?;

                        let res = fnearest!(F32, value)?;
                        current_frame.push(res);
                    }
                    Instr::F32Sqrt => {
                        let value = current_frame.pop()?;

                        let res = fsqrt!(F32, value)?;
                        current_frame.push(res);
                    }


                    Instr::F32Add => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = fadd!(F32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::F32Sub => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = fsub!(F32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::F32Mul => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = fmul!(F32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::F32Div => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = fdiv!(F32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::F32Min => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = fmin!(F32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::F32Max => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = fmax!(F32, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::F32Copysign => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = fcopysign!(F32, lhs, rhs)?;
                        current_frame.push(res);
                    }

                    // F64 BINOP / UNOP
                    Instr::F64Abs => {
                        let value = current_frame.pop()?;

                        let res = fabs!(F64, value)?;
                        current_frame.push(res);
                    }
                    Instr::F64Neg => {
                        let value = current_frame.pop()?;

                        let res = fneg!(F64, value)?;
                        current_frame.push(res);
                    }
                    Instr::F64Ceil => {
                        let value = current_frame.pop()?;

                        let res = fceil!(F64, value)?;
                        current_frame.push(res);
                    }
                    Instr::F64Floor => {
                        let value = current_frame.pop()?;

                        let res = ffloor!(F64, value)?;
                        current_frame.push(res);
                    }
                    Instr::F64Trunc => {
                        let value = current_frame.pop()?;

                        let res = ftrunc!(F64, value)?;
                        current_frame.push(res);
                    }
                    Instr::F64Nearest => {
                        let value = current_frame.pop()?;

                        let res = fnearest!(F64, value)?;
                        current_frame.push(res);
                    }
                    Instr::F64Sqrt => {
                        let value = current_frame.pop()?;

                        let res = fsqrt!(F64, value)?;
                        current_frame.push(res);
                    }


                    Instr::F64Add => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = fadd!(F64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::F64Sub => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = fsub!(F64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::F64Mul => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = fmul!(F64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::F64Div => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = fdiv!(F64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::F64Min => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = fmin!(F64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::F64Max => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = fmax!(F64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::F64Copysign => {
                        let (lhs, rhs) = current_frame.pop_pair()?;

                        let res = fcopysign!(F64, lhs, rhs)?;
                        current_frame.push(res);
                    }
                    Instr::I32WrapI64 => {
                        let value = current_frame.pop()?;

                        let res = math::wrap(value)?;
                        current_frame.push(res);
                    }
                    // I32 Trunc

                    Instr::I32TruncF32S => {
                        let value = current_frame.pop()?;

                        let res = trunc!(I32, F32, i32, value)?;
                        current_frame.push(res);
                    }
                    Instr::I32TruncF32U => {
                        let value = current_frame.pop()?;

                        let res = trunc!(I32, F32, u32, value)?;
                        current_frame.push(res);
                    }
                    Instr::I32TruncF64S => {
                        let value = current_frame.pop()?;

                        let res = trunc!(I32, F64, i32, value)?;
                        current_frame.push(res);
                    }
                    Instr::I32TruncF64U => {
                        let value = current_frame.pop()?;

                        let res = trunc!(I32, F64, u32, value)?;
                        current_frame.push(res);
                    }

                    // I64 Extend
                    Instr::I64ExtendI32S => {
                        let value = current_frame.pop()?;

                        let res = math::iextend(value, true)?;
                        current_frame.push(res);
                    }
                    Instr::I64ExtendI32U => {
                        let value = current_frame.pop()?;

                        let res = math::iextend(value, false)?;
                        current_frame.push(res);
                    }

                    // I64 Trunc
                    Instr::I64TruncF32S => {
                        let value = current_frame.pop()?;

                        let res = trunc!(I64, F32, i32, value)?;
                        current_frame.push(res);
                    }
                    Instr::I64TruncF32U => {
                        let value = current_frame.pop()?;

                        let res = trunc!(I64, F32, u32, value)?;
                        current_frame.push(res);
                    }
                    Instr::I64TruncF64S => {
                        let value = current_frame.pop()?;

                        let res = trunc!(I64, F64, i32, value)?;
                        current_frame.push(res);
                    }
                    Instr::I64TruncF64U => {
                        let value = current_frame.pop()?;

                        let res = trunc!(I64, F64, u32, value)?;
                        current_frame.push(res);
                    }

                    // F32 Convert
                    Instr::F32ConvertI32S => {
                        let value = current_frame.pop()?;

                        let res = convert!(I32 => f32, value, i32)?;
                        current_frame.push(res);
                    }
                    Instr::F32ConvertI32U => {
                        let value = current_frame.pop()?;

                        let res = convert!(I32 => f32, value)?;
                        current_frame.push(res);
                    }
                    Instr::F32ConvertI64S => {
                        let value = current_frame.pop()?;

                        let res = convert!(I64 => f32, value, i32)?;
                        current_frame.push(res);
                    }
                    Instr::F32ConvertI64U => {
                        let value = current_frame.pop()?;

                        let res = convert!(I64 => f32, value)?;
                        current_frame.push(res);
                    }

                    // Demote
                    Instr::F32DemoteF64 => {
                        let value = current_frame.pop()?;

                        let res = math::demote(value)?;
                        current_frame.push(res);
                    }

                    // F64 Convert
                    Instr::F64ConvertI32S => {
                        let value = current_frame.pop()?;

                        let res = convert!(I32 => f64, value, i32)?;
                        current_frame.push(res);
                    }
                    Instr::F64ConvertI32U => {
                        let value = current_frame.pop()?;

                        let res = convert!(I32 => f64, value)?;
                        current_frame.push(res);
                    }
                    Instr::F64ConvertI64S => {
                        let value = current_frame.pop()?;

                        let res = convert!(I64 => f64, value, i32)?;
                        current_frame.push(res);
                    }
                    Instr::F64ConvertI64U => {
                        let value = current_frame.pop()?;

                        let res = convert!(I64 => f64, value)?;
                        current_frame.push(res);
                    }

                    // Promote
                    Instr::F64PromoteF32 => {
                        let value = current_frame.pop()?;

                        let res = math::promote(value)?;
                        current_frame.push(res);
                    }

                    // Reinterp
                    Instr::I32ReinterpF32 => {
                        let value = current_frame.pop()?;

                        let res = reinterp!(I32, value)?;
                        current_frame.push(res);
                    }
                    Instr::F32ReinterpI32 => {
                        let value = current_frame.pop()?;

                        let res = reinterp!(F32, value)?;
                        current_frame.push(res);
                    }
                    Instr::I64ReinterpF64 => {
                        let value = current_frame.pop()?;

                        let res = reinterp!(I64, value)?;
                        current_frame.push(res);
                    }
                    Instr::F64ReinterpI64 => {
                        let value = current_frame.pop()?;

                        let res = reinterp!(F64, value)?;
                        current_frame.push(res);
                    }
                    instr => return Err(Error::NotImplemented(instr.clone())),
                }

                debug!("\t---> [Locals] {:?}", current_frame.locals);
                debug!("\t---> [Values] {:#?}", current_frame.stack);
                debug!("\t---> [Labels] {:?}", current_frame.label_stack);
                debug!("");
            }

            if self.frames.len() == 0 {
                let result = match current_frame.func.res().clone() {
                    ResType::Unit => Ok(WasmResult::Unit),
                    res => match (res, current_frame.pop()?) {
                        (ResType::I32, r @ Value::I32(_)) => Ok(WasmResult::from(r)),
                        (ResType::I64, r @ Value::I64(_)) => Ok(WasmResult::from(r)),
                        (ResType::F32, r @ Value::F32(_)) => Ok(WasmResult::from(r)),
                        (ResType::F64, r @ Value::F64(_)) => Ok(WasmResult::from(r)),
                        _ => Err(Error::TypeMismatch(line!())),
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
                            _ => Err(Error::TypeMismatch(line!())),
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
