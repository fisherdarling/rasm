use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::ops::{Deref, DerefMut};

use crate::error::{Error, ExecResult};

use crate::function::{FuncReader, FuncRef, Function};
use crate::instr::{Expression, Instr};
use crate::runtime::frame::{Frame, LabelType, ValueStack};
use crate::types::{
    index::{FuncIdx, Offset, LabelIdx},
    Mut, ResType, ValType, Value, WasmResult,
};
// use crate::{binop, is_a, relop, same_type, truthy, valid_result};
use crate::math;
use crate::store::Store;
use crate::*;

use crate::store::memory::MemInst;

use log::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InstrResult {
    Goto {  loc: usize, 
            clean_depth: Option<usize>,
            value: Option<Value>, },
    Clean { clean_depth: Option<usize>, 
            value: Option<Value>, },
    Call(FuncIdx),
    Return,
    Continue,
}

#[derive(Debug)]
pub struct Interpreter<'a> {
    frames: FrameStack,
    stack: ValueStack,
    resolver: &'a HashMap<String, FuncIdx>,
    store: &'a mut Store,
}

impl Interpreter<'_> {
    pub fn new<'a>(
        resolver: &'a HashMap<String, FuncIdx>,
        store: &'a mut Store,
    ) -> Interpreter<'a> {
        Interpreter {
            resolver,
            store,
            frames: FrameStack::default(),
            stack: ValueStack::default(),
            // current_frame: None,
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
        let function: &FuncRef = self
            .store
            .functions
            .get(func_idx.as_usize())
            .ok_or(Error::InvalidFuncIdx(*func_idx))?;

        let frame = function.instantiate(args)?;

        self.execute_with_frame(frame)
    }

    pub fn invoke_index<A: AsRef<[Value]>>(
        &mut self,
        idx: usize,
        args: A,
    ) -> ExecResult<WasmResult> {
        let args: &[Value] = args.as_ref();

        let function: &FuncRef = self
            .store
            .functions
            .get(idx)
            .ok_or(Error::InvalidFuncIdx((idx as u32).into()))?;

        let frame = function.instantiate(args)?;

        self.execute_with_frame(frame)
    }

    #[inline]
    fn execute_with_frame(&mut self, frame: Frame) -> ExecResult<WasmResult> {
        self.push_frame(frame);

        self.execute()
    }

    // pub fn print_stack(&self) {
    // }

    pub fn execute(&mut self) -> ExecResult<WasmResult> {
        'frame: loop {
            let mut current_frame: &mut Frame = self.current_frame()?;
            let frame_res = current_frame.res();

            let mut reader = if current_frame.is_paused() {
                current_frame.unpause()?
            } else {
                current_frame.func.reader()
            };

            'instr: loop {
                // TODO Handle None case

                let pos = reader.pos().unwrap_or_default();
                let next_instr: Option<&Instr> = reader.next();

                if next_instr.is_none() {
                    break;
                }

                let next_instr = next_instr.unwrap();

                debug!("[{:?}] {:?}", pos, next_instr);
                // debug!("\t---> [Lables] {:?}", current_frame.clone().label_stack);

                let instruction_result = self.execute_instr(next_instr)?;

                debug!("\t---> [Values] {:?}", self.stack);
                // debug!("\t---> [Locals] {:?}", current_frame.locals);

                match instruction_result {
                    InstrResult::Goto { loc, clean_depth, value } => {
                        debug!("[GOTO] Instr: {:?}, Clean: {:?}, Value: {:?}", loc, clean_depth, value);
                        
                        self.stack.pop_label_depth(clean_depth);
                        
                        if let Some(value) = value {
                            self.stack.push(value);
                        }

                        debug!("[GOTO] Stack after clean {:?}", self.stack);

                        reader.goto(loc);
                    }
                    InstrResult::Clean { clean_depth, value } => {
                        debug!("[CLEAN] Clean: {:?}, Value: {:?}", clean_depth, value);
                        
                        self.stack.pop_label_depth(clean_depth);
                        
                        if let Some(value) = value {
                            self.stack.push(value);
                        }

                        debug!("[CLEAN] Stack after clean {:?}", self.stack);
                    }
                    InstrResult::Call(idx) => {
                        debug!("== [CALL ({:?})] ==", idx);

                        let function = self
                            .store
                            .functions
                            .get(idx.as_usize())
                            .ok_or(Error::InvalidFuncIdx(idx.clone()))?;

                        let num_values = function.argument_length();
                        let args = self.stack.pop_args(num_values)?;
                        let new_frame = function.instantiate(&args)?;

                        self.current_frame()?.pause(reader);
                        self.push_frame(new_frame);

                        continue 'frame;
                    }
                    InstrResult::Return => {
                        debug!("== [RETURN] ==");

                        break 'instr;
                    }
                    InstrResult::Continue => continue 'instr,
                }
            }

            
            // let clean_depth = current_frame.label_stack.len().clone();

            if self.frames.len() == 1 {
                let result = match frame_res {
                    ResType::Unit => Ok(WasmResult::Unit),
                    res => {
                        let value = self.stack.pop()?;
                        valid_result!(res from value)
                    }
                };

                debug!("Returning to outer function: {:?}", result);

                return result;
            } else {
                let value = match frame_res {
                    ResType::Unit => None,
                    res => {
                        let value = self.stack.pop()?;
                        valid_result!(res, value)?;
                        Some(value)
                    }
                };

                let len = self.current_frame()?.label_stack.len();

                let clean_depth = if len > 0 {
                    Some(len - 1)
                } else {
                    None
                };

                self.stack.pop_label_depth(clean_depth);

                if let Some(value) = value {
                    self.stack.push(value);
                }
            }

            self.pop_frame();
        }
    }

    fn execute_instr(&mut self, instr: &Instr) -> ExecResult<InstrResult> {
        debug!("\t---> [Locals] {:?}", self.current_frame()?.locals);
        debug!("\t---> [Labels] {:?}", self.current_frame()?.label_stack);

        match instr {
            Instr::Drop => {
                let _ = self.stack.pop()?;
            }
            Instr::Nop => {}
            Instr::Unreachable => {
                return Err(Error::TrapUnreachable);
            }
            Instr::IfMarker(result, true_end, false_end) => {
                let check = self.stack.pop()?;
                self.current_frame()?
                    .label_stack
                    .push(LabelType::If(*result, *true_end, *false_end));
                
                self.stack.push_label();

                debug!("\t---> [IF_MARKER] Check: {:?}, truthy!: {:?}", check, truthy!(check));

                if truthy!(check)? {
                    // Do nothing, the next instruction is part of the truthy path.
                } else {
                    // Skip the true path, nothing to clean since we're entering a scope.
                    return Ok(InstrResult::Goto { loc: *true_end, clean_depth: None, value: None });
                }
            }
            Instr::BlockMarker(result, block_end) => {
                self.current_frame()?
                    .label_stack
                    .push(LabelType::Block(*result, *block_end));
                
                self.stack.push_label();
            }
            Instr::LoopMarker(result, loop_start) => {
                self.current_frame()?
                    .label_stack
                    .push(LabelType::Loop(*result, *loop_start));

                self.stack.push_label();
            }
            Instr::Select => {
                let check = self.stack.pop()?;
                let val_2 = self.stack.pop()?;
                let val_1 = self.stack.pop()?;

                same_type!(val_1, val_2)?;

                if truthy!(check)? {
                    self.stack.push(val_1);
                } else {
                    self.stack.push(val_2);
                }
            }
            Instr::Br(idx) => {
                let mut inner_result;
                
                if let Some(label) = self.current_frame()?.label_stack.last() {
                    inner_result = label.res();
                } else {
                    return Ok(InstrResult::Return);
                }

                for i in 0..**idx {
                    self.current_frame()?
                        .label_stack
                        .pop()
                        .ok_or(Error::BranchDepth(*idx))?;
                }

                if self.current_frame()?.label_stack.is_empty() {
                    return Ok(InstrResult::Return);
                }

                let target = self
                    .current_frame()?
                    .label_stack
                    .pop()
                    .ok_or(Error::BranchDepth(*idx))?;

                debug!("\t---> Inner Result: {:?}, Target: {:?}", inner_result, target);

                match target {
                    LabelType::Block(outer_result, block_end) => {
                        let value = match outer_result {
                            ResType::Unit => None,
                            res => {
                                let value = self.stack.pop()?;
                                valid_result!(res, value)?;
                                Some(value)
                            }
                        };

                        // if value.is_some() && inner_result != outer_result {
                        //     return Err(Error::TypeMismatch(line!()));
                        // }

                        let clean_depth = Some(idx.as_usize());

                        return Ok(InstrResult::Goto { loc: block_end, clean_depth, value });
                    },
                    LabelType::If(outer_result, true_end, false_end) => {
                        let value = match outer_result {
                            ResType::Unit => None,
                            res => {
                                let value = self.stack.pop()?;
                                valid_result!(res, value)?;
                                Some(value)
                            }
                        };

                        // if value.is_some() && inner_result != outer_result {
                        //     return Err(Error::TypeMismatch(line!()));
                        // }

                        let clean_depth = Some(idx.as_usize());

                        return Ok(InstrResult::Goto { loc: false_end, clean_depth, value });
                    },
                    LabelType::Loop(loop_result, loop_start) => {
                        if idx.as_usize() == 0 {
                            // We're just repeating the loop;

                            self.current_frame()?.label_stack.push(LabelType::Loop(loop_result, loop_start));

                            return Ok(InstrResult::Goto { loc: loop_start, clean_depth: None, value: None });
                        }
                        
                        let value = match inner_result {
                            ResType::Unit => None,
                            res => {
                                let value = self.stack.pop()?;
                                valid_result!(res, value)?;
                                Some(value)
                            }
                        };

                        let clean_depth = Some(idx.as_usize());

                        self.current_frame()?.label_stack.push(LabelType::Loop(loop_result, loop_start));

                        return Ok(InstrResult::Goto { loc: loop_start, clean_depth, value });
                    },
                }
            }
            Instr::BrIf(idx) => {
                let check = self.stack.pop()?;

                if !truthy!(check)? {
                    debug!("\t---> BrIf NOP");
                    return Ok(InstrResult::Continue);
                }

                let mut inner_result;

                if let Some(label) = self.current_frame()?.label_stack.last() {
                    inner_result = label.res();
                } else {
                    return Ok(InstrResult::Return);
                }

                for i in 0..**idx {
                    self.current_frame()?
                        .label_stack
                        .pop()
                        .ok_or(Error::BranchDepth(*idx))?;
                }

                if self.current_frame()?.label_stack.is_empty() {
                    return Ok(InstrResult::Return);
                }

                let target = self
                    .current_frame()?
                    .label_stack
                    .pop()
                    .ok_or(Error::BranchDepth(*idx))?;
                debug!("\t---> Inner Result: {:?}, Target: {:?}", inner_result, target);
                
                match target {
                    LabelType::Block(outer_result, block_end) => {
                        let value = match outer_result {
                            ResType::Unit => None,
                            res => {
                                let value = self.stack.pop()?;
                                valid_result!(res, value)?;
                                Some(value)
                            }
                        };

                        // if value.is_some() && inner_result != outer_result {
                        //     return Err(Error::TypeMismatch(line!()));
                        // }

                        let clean_depth = Some(idx.as_usize());

                        return Ok(InstrResult::Goto { loc: block_end, clean_depth, value });
                    },
                    LabelType::If(outer_result, true_end, false_end) => {
                        let value = match outer_result {
                            ResType::Unit => None,
                            res => {
                                let value = self.stack.pop()?;
                                valid_result!(res, value)?;
                                Some(value)
                            }
                        };

                        // if value.is_some() && inner_result != outer_result {
                        //     return Err(Error::TypeMismatch(line!()));
                        // }

                        let clean_depth = Some(idx.as_usize());

                        return Ok(InstrResult::Goto { loc: false_end, clean_depth, value });
                    },
                    LabelType::Loop(loop_result, loop_start) => {
                        if idx.as_usize() == 0 {
                            // We're just repeating the loop;

                            self.current_frame()?.label_stack.push(LabelType::Loop(loop_result, loop_start));

                            return Ok(InstrResult::Goto { loc: loop_start, clean_depth: None, value: None });
                        }
                        
                        let value = match inner_result {
                            ResType::Unit => None,
                            res => {
                                let value = self.stack.pop()?;
                                valid_result!(res, value)?;
                                Some(value)
                            }
                        };

                        let clean_depth = Some(idx.as_usize());

                        self.current_frame()?.label_stack.push(LabelType::Loop(loop_result, loop_start));

                        return Ok(InstrResult::Goto { loc: loop_start, clean_depth, value });
                    },
                }
            }
            Instr::BrTable(table, default) => {
                let idx = get!(I32, self.stack.pop()?)? as usize;

                // println!("Len: {:?}, Default: {:?}, Idx: {:?}", table.len(), default, idx);

                let idx = if idx < table.len() {
                    table[idx].as_usize()
                } else {
                    default.as_usize()
                };

                // println!("Chosen Idx: {:?}", idx);


                let mut inner_result;

                if let Some(label) = self.current_frame()?.label_stack.last() {
                    inner_result = label.res();
                } else {
                    return Ok(InstrResult::Return);
                }

                for i in 0..idx {
                    self.current_frame()?
                        .label_stack
                        .pop()
                        .ok_or(Error::BranchDepth(LabelIdx::from(idx as u32)))?;
                }

                if self.current_frame()?.label_stack.is_empty() {
                    return Ok(InstrResult::Return);
                }

                let target = self
                    .current_frame()?
                    .label_stack
                    .pop()
                    .ok_or(Error::BranchDepth(LabelIdx::from(idx as u32)))?;

                debug!("\t---> Inner Result: {:?}, Target: {:?}", inner_result, target);

                match target {
                    LabelType::Block(outer_result, block_end) => {
                        let value = match outer_result {
                            ResType::Unit => None,
                            res => {
                                let value = self.stack.pop()?;
                                valid_result!(res, value)?;
                                Some(value)
                            }
                        };

                        // if value.is_some() && inner_result != outer_result {
                        //     return Err(Error::TypeMismatch(line!()));
                        // }

                        let clean_depth = Some(idx);

                        return Ok(InstrResult::Goto { loc: block_end, clean_depth, value });
                    },
                    LabelType::If(outer_result, true_end, false_end) => {
                        let value = match outer_result {
                            ResType::Unit => None,
                            res => {
                                let value = self.stack.pop()?;
                                valid_result!(res, value)?;
                                Some(value)
                            }
                        };

                        // if value.is_some() && inner_result != outer_result {
                        //     return Err(Error::TypeMismatch(line!()));
                        // }

                        let clean_depth = Some(idx);

                        return Ok(InstrResult::Goto { loc: false_end, clean_depth, value });
                    },
                    LabelType::Loop(loop_result, loop_start) => {
                        if idx == 0 {
                            // We're just repeating the loop;

                            self.current_frame()?.label_stack.push(LabelType::Loop(loop_result, loop_start));

                            return Ok(InstrResult::Goto { loc: loop_start, clean_depth: None, value: None });
                        }
                        
                        let value = match inner_result {
                            ResType::Unit => None,
                            res => {
                                let value = self.stack.pop()?;
                                valid_result!(res, value)?;
                                Some(value)
                            }
                        };

                        let clean_depth = Some(idx);

                        self.current_frame()?.label_stack.push(LabelType::Loop(loop_result, loop_start));

                        return Ok(InstrResult::Goto { loc: loop_start, clean_depth, value });
                    },
                }
            }
            Instr::Call(idx) => return Ok(InstrResult::Call(*idx)),
            Instr::Return => return Ok(InstrResult::Return),
            Instr::End => {
                // debug!("[End], Reader: {:?}, Scope: {:?}", reader.finished(), self.current_frame()?.label_stack);

                if self.current_frame()?.label_stack.is_empty() {
                    // assert!(reader.finished());
                    return Ok(InstrResult::Return);
                }

                let outer_label = self
                    .current_frame()?
                    .label_stack
                    .pop()
                    .expect("Label stack must contain a label");

                debug!("\t---> [END] Target: {:?}", outer_label);
                
                match outer_label {
                    LabelType::If(result, true_end, false_end) => {
                        if result == ResType::Unit {
                            return Ok(InstrResult::Goto { loc: false_end, clean_depth: Some(0), value: None });
                        } else {
                            let value = self.stack.pop()?;
                            valid_result!(result, value)?;
                            return Ok(InstrResult::Goto { loc: false_end, clean_depth: Some(0), value: Some(value) });
                        }
                    }
                    LabelType::Block(result, block_end) => {
                        if result == ResType::Unit {
                            return Ok(InstrResult::Clean { clean_depth: Some(0), value: None });
                        } else {
                            let value = self.stack.pop()?;
                            valid_result!(result, value)?;
                            return Ok(InstrResult::Clean { clean_depth: Some(0), value: Some(value) });
                        }
                    }

                    // FIXME: LOOP END IS A BRANCH
                    LabelType::Loop(result, loop_end) => {
                        if result == ResType::Unit {
                            return Ok(InstrResult::Clean { clean_depth: Some(0), value: None });
                        } else {
                            let value = self.stack.pop()?;
                            valid_result!(result, value)?;
                            return Ok(InstrResult::Clean { clean_depth: Some(0), value: Some(value) });
                        }
                    }
                }
            }
            Instr::LocalGet(idx) => {
                let value = self.current_frame()?[idx.index() as usize];
                self.stack.push(value);
            }
            Instr::LocalSet(idx) => {
                let value = self.stack.pop()?;
                self.current_frame()?[idx.index() as usize] = value;
            }
            Instr::LocalTee(idx) => {
                let value = self.stack.pop()?;

                self.stack.push(value.clone());

                self.current_frame()?[idx.index() as usize] = value;
            }
            Instr::GlobalGet(idx) => {
                let global = self
                    .store
                    .globals
                    .get(idx.as_usize())
                    .ok_or(Error::GlobalIndex(idx.clone()))?;

                self.stack.push(global.value);
            }

            Instr::GlobalSet(idx) => {
                let global = self
                    .store
                    .globals
                    .get_mut(idx.as_usize())
                    .ok_or(Error::GlobalIndex(idx.clone()))?;

                if let Mut::Var = global.var {
                    let new_value = self.stack.pop()?;
                    let prev = global.get();

                    same_type!(new_value, prev)?;

                    global.set(new_value);
                } else {
                    return Err(Error::GlobalMut(idx.clone()));
                }
            }

            // I32/64 Full Load
            Instr::I32Load(align, offset) => {
                let offset = self.get_delta(*offset)?;
                let res = self.store.memory.load_i32(align.clone(), offset.clone());
                self.stack.push(Value::from(res));
            }
            Instr::I64Load(align, offset) => {
                let offset = self.get_delta(*offset)?;
                let res = self.store.memory.load_i64(align.clone(), offset.clone());
                self.stack.push(Value::from(res));
            }

            // F32/64 Full Load
            Instr::F32Load(align, offset) => {
                let offset = self.get_delta(*offset)?;
                let res = self.store.memory.load_f32(align.clone(), offset.clone());
                self.stack.push(Value::from(res));
            }
            Instr::F64Load(align, offset) => {
                let offset = self.get_delta(*offset)?;
                let res = self.store.memory.load_f64(align.clone(), offset.clone());
                self.stack.push(Value::from(res));
            }

            // I32 Partial Mem Load
            Instr::I32Load8S(align, offset) => {
                let offset = self.get_delta(*offset)?;
                let res = self
                    .store
                    .memory
                    .load_i32_8_s(align.clone(), offset.clone());
                self.stack.push(Value::from(res));
            }
            Instr::I32Load8U(align, offset) => {
                let offset = self.get_delta(*offset)?;
                let res = self
                    .store
                    .memory
                    .load_i32_8_u(align.clone(), offset.clone());
                self.stack.push(Value::from(res));
            }
            Instr::I32Load16S(align, offset) => {
                let offset = self.get_delta(*offset)?;
                let res = self
                    .store
                    .memory
                    .load_i32_16_s(align.clone(), offset.clone());
                self.stack.push(Value::from(res));
            }
            Instr::I32Load16U(align, offset) => {
                let offset = self.get_delta(*offset)?;
                let res = self
                    .store
                    .memory
                    .load_i32_16_u(align.clone(), offset.clone());
                self.stack.push(Value::from(res));
            }

            // I64 Partial Mem Load
            Instr::I64Load8S(align, offset) => {
                let offset = self.get_delta(*offset)?;
                let res = self
                    .store
                    .memory
                    .load_i64_8_s(align.clone(), offset.clone());
                self.stack.push(Value::from(res));
            }
            Instr::I64Load8U(align, offset) => {
                let offset = self.get_delta(*offset)?;
                let res = self
                    .store
                    .memory
                    .load_i64_8_u(align.clone(), offset.clone());
                self.stack.push(Value::from(res));
            }
            Instr::I64Load16S(align, offset) => {
                let offset = self.get_delta(*offset)?;
                let res = self
                    .store
                    .memory
                    .load_i64_16_s(align.clone(), offset.clone());
                self.stack.push(Value::from(res));
            }
            Instr::I64Load16U(align, offset) => {
                let offset = self.get_delta(*offset)?;
                let res = self
                    .store
                    .memory
                    .load_i64_16_u(align.clone(), offset.clone());
                self.stack.push(Value::from(res));
            }
            Instr::I64Load32S(align, offset) => {
                let offset = self.get_delta(*offset)?;
                let res = self
                    .store
                    .memory
                    .load_i64_32_s(align.clone(), offset.clone());
                self.stack.push(Value::from(res));
            }
            Instr::I64Load32U(align, offset) => {
                let offset = self.get_delta(*offset)?;
                let res = self
                    .store
                    .memory
                    .load_i64_32_u(align.clone(), offset.clone());
                self.stack.push(Value::from(res));
            }

            // Memory Storage Operators
            Instr::I32Store(align, offset) => {
                let value = get!(I32, self.stack.pop()?)?;
                let offset = self.get_delta(*offset)?;
                self.store.memory.store_i32(*align, offset, value);
            }
            Instr::I64Store(align, offset) => {
                let value = get!(I64, self.stack.pop()?)?;
                let offset = self.get_delta(*offset)?;
                self.store.memory.store_i64(*align, offset, value);
            }
            Instr::F32Store(align, offset) => {
                let value = get!(F32, self.stack.pop()?)?;
                let offset = self.get_delta(*offset)?;
                self.store.memory.store_f32(*align, offset, value);
            }
            Instr::F64Store(align, offset) => {
                let value = get!(F64, self.stack.pop()?)?;
                let offset = self.get_delta(*offset)?;
                self.store.memory.store_f64(*align, offset, value);
            }
            Instr::I32Store8(align, offset) => {
                let value = get!(I32, self.stack.pop()?)?;
                let offset = self.get_delta(*offset)?;
                self.store.memory.store_i32_8(*align, offset, value);
            }
            Instr::I32Store16(align, offset) => {
                let value = get!(I32, self.stack.pop()?)?;
                let offset = self.get_delta(*offset)?;
                self.store.memory.store_i32_16(*align, offset, value);
            }
            Instr::I64Store8(align, offset) => {
                let value = get!(I64, self.stack.pop()?)?;
                let offset = self.get_delta(*offset)?;
                self.store.memory.store_i64_8(*align, offset, value);
            }
            Instr::I64Store16(align, offset) => {
                let value = get!(I64, self.stack.pop()?)?;
                let offset = self.get_delta(*offset)?;
                self.store.memory.store_i64_16(*align, offset, value);
            }
            Instr::I64Store32(align, offset) => {
                let value = get!(I64, self.stack.pop()?)?;
                let offset = self.get_delta(*offset)?;
                self.store.memory.store_i64_32(*align, offset, value);
            }
            Instr::MemSize(_reserved) => {
                let size = self.store.memory.mem_size() as i32;
                self.stack.push(Value::from(size));
            }
            Instr::MemGrow(_reserved) => {
                let num_pages = get!(I32, self.stack.pop()?)?;
                let old_size = self.store.memory.mem_grow(num_pages)?;
                self.stack.push(Value::from(old_size));
            }

            // Const operators
            Instr::I32Const(c) => {
                self.stack.push(Value::I32(*c as i32));
            }
            Instr::I64Const(c) => {
                self.stack.push(Value::I64(*c as i64));
            }
            Instr::F32Const(c) => {
                self.stack.push(Value::F32(*c));
            }
            Instr::F64Const(c) => {
                self.stack.push(Value::F64(*c));
            }
            // I32 RELOP
            Instr::I32Eqz => {
                let val = is_a!(I32, self.stack.pop())?;

                let res = Value::from(val == Value::I32(0));
                self.stack.push(res);
            }
            Instr::I32Eq => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = ieq!(I32, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I32Ne => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = ine!(I32, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I32GtS => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = igt_s!(I32, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I32GtU => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = igt_u!(I32, lhs, rhs, u32)?;
                self.stack.push(res);
            }
            Instr::I32LtS => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = ilt_s!(I32, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I32LtU => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = ilt_u!(I32, lhs, rhs, u32)?;
                self.stack.push(res);
            }
            Instr::I32LeU => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = ile_u!(I32, lhs, rhs, u32)?;
                self.stack.push(res);
            }
            Instr::I32LeS => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = ile_s!(I32, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I32GeU => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = ige_u!(I32, lhs, rhs, u32)?;
                self.stack.push(res);
            }
            Instr::I32GeS => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = ige_s!(I32, lhs, rhs)?;
                self.stack.push(res);
            }

            // I64 RELOP
            Instr::I64Eqz => {
                let val = is_a!(I64, self.stack.pop())?;

                let res = Value::from(val == Value::I64(0));
                self.stack.push(res);
            }
            Instr::I64Eq => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = ieq!(I64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I64Ne => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = ine!(I64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I64GtS => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = igt_s!(I64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I64GtU => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = igt_u!(I64, lhs, rhs, u64)?;
                self.stack.push(res);
            }
            Instr::I64LtS => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = ilt_s!(I64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I64LtU => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = ilt_u!(I64, lhs, rhs, u64)?;
                self.stack.push(res);
            }
            Instr::I64LeU => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = ile_u!(I64, lhs, rhs, u64)?;
                self.stack.push(res);
            }
            Instr::I64LeS => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = ile_s!(I64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I64GeU => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = ige_u!(I64, lhs, rhs, u64)?;
                self.stack.push(res);
            }
            Instr::I64GeS => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = ige_s!(I64, lhs, rhs)?;
                self.stack.push(res);
            }

            // F32 RELOP
            Instr::F32Eq => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = feq!(F32, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::F32Ne => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = fne!(F32, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::F32Lt => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = flt!(F32, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::F32Gt => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = fgt!(F32, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::F32Le => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = fle!(F32, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::F32Ge => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = fge!(F32, lhs, rhs)?;
                self.stack.push(res);
            }

            // F64 RELOP
            Instr::F64Eq => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = feq!(F64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::F64Ne => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = fne!(F64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::F64Lt => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = flt!(F64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::F64Gt => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = fgt!(F64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::F64Le => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = fle!(F64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::F64Ge => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = fge!(F64, lhs, rhs)?;
                self.stack.push(res);
            }

            // I32 BINOP / UNOP, TODO SHL/R(S/U) ROTL/ROTR
            Instr::I32Clz => {
                let value = self.stack.pop()?;

                let res = iclz!(I32, value)?;
                self.stack.push(res);
            }
            Instr::I32Ctz => {
                let value = self.stack.pop()?;

                let res = ictz!(I32, value)?;
                self.stack.push(res);
            }
            Instr::I32Popcnt => {
                let value = self.stack.pop()?;

                let res = ipopcnt!(I32, value)?;
                self.stack.push(res);
            }
            Instr::I32Add => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = iadd!(I32, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I32Sub => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = isub!(I32, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I32Mul => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = imul!(I32, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I32DivS => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = idiv_s!(I32, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I32DivU => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = idiv_u!(I32, lhs, rhs, u32)?;
                self.stack.push(res);
            }

            Instr::I32RemS => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = irem_s!(I32, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I32RemU => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = irem_u!(I32, lhs, rhs, u32)?;
                self.stack.push(res);
            }
            Instr::I32And => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = iand!(I32, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I32Or => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = ior!(I32, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I32Xor => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = ixor!(I32, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I32Shl => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = shl!(I32, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I32ShrS => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = shr!(I32, lhs, rhs, i32)?;
                self.stack.push(res);
            }
            Instr::I32ShrU => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                // println!("LHS: {:?}, RHS: {:?}", lhs, rhs);

                let res = shr!(I32, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I32Rotl => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = rotl!(I32, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I32Rotr => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = rotr!(I32, lhs, rhs)?;
                self.stack.push(res);
            }

            // I64 BINOP / UNOP TODO SHL/R(S/U) ROTL/ROTR
            Instr::I64Clz => {
                let value = self.stack.pop()?;

                let res = iclz!(I64, value)?;
                self.stack.push(res);
            }
            Instr::I64Ctz => {
                let value = self.stack.pop()?;

                let res = ictz!(I64, value)?;
                self.stack.push(res);
            }
            Instr::I64Popcnt => {
                let value = self.stack.pop()?;

                let res = ipopcnt!(I64, value)?;
                self.stack.push(res);
            }
            Instr::I64Add => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = iadd!(I64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I64Sub => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = isub!(I64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I64Mul => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = imul!(I64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I64DivS => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = idiv_s!(I64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I64DivU => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = idiv_u!(I64, lhs, rhs, u64)?;
                self.stack.push(res);
            }

            Instr::I64RemS => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = irem_s!(I64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I64RemU => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = irem_u!(I64, lhs, rhs, u64)?;
                self.stack.push(res);
            }
            Instr::I64And => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = iand!(I64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I64Or => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = ior!(I64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I64Xor => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = ixor!(I64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I64Shl => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = shl!(I64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I64ShrS => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = shr!(I64, lhs, rhs, i32)?;
                self.stack.push(res);
            }
            Instr::I64ShrU => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = shr!(I64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I64Rotl => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = rotl!(I64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I64Rotr => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = rotr!(I64, lhs, rhs)?;
                self.stack.push(res);
            }

            // F32 BINOP / UNOP
            Instr::F32Abs => {
                let value = self.stack.pop()?;

                let res = fabs!(F32, value)?;
                self.stack.push(res);
            }
            Instr::F32Neg => {
                let value = self.stack.pop()?;

                let res = fneg!(F32, value)?;
                self.stack.push(res);
            }
            Instr::F32Ceil => {
                let value = self.stack.pop()?;

                let res = fceil!(F32, value)?;
                self.stack.push(res);
            }
            Instr::F32Floor => {
                let value = self.stack.pop()?;

                let res = ffloor!(F32, value)?;
                self.stack.push(res);
            }
            Instr::F32Trunc => {
                let value = self.stack.pop()?;

                let res = ftrunc!(F32, value)?;
                self.stack.push(res);
            }
            Instr::F32Nearest => {
                let value = self.stack.pop()?;

                let res = fnearest!(F32, value)?;
                self.stack.push(res);
            }
            Instr::F32Sqrt => {
                let value = self.stack.pop()?;

                let res = fsqrt!(F32, value)?;
                self.stack.push(res);
            }

            Instr::F32Add => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = fadd!(F32, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::F32Sub => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = fsub!(F32, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::F32Mul => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = fmul!(F32, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::F32Div => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = fdiv!(F32, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::F32Min => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = fmin!(F32, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::F32Max => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = fmax!(F32, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::F32Copysign => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = fcopysign!(F32, lhs, rhs)?;
                self.stack.push(res);
            }

            // F64 BINOP / UNOP
            Instr::F64Abs => {
                let value = self.stack.pop()?;

                let res = fabs!(F64, value)?;
                self.stack.push(res);
            }
            Instr::F64Neg => {
                let value = self.stack.pop()?;

                let res = fneg!(F64, value)?;
                self.stack.push(res);
            }
            Instr::F64Ceil => {
                let value = self.stack.pop()?;

                let res = fceil!(F64, value)?;
                self.stack.push(res);
            }
            Instr::F64Floor => {
                let value = self.stack.pop()?;

                let res = ffloor!(F64, value)?;
                self.stack.push(res);
            }
            Instr::F64Trunc => {
                let value = self.stack.pop()?;

                let res = ftrunc!(F64, value)?;
                self.stack.push(res);
            }
            Instr::F64Nearest => {
                let value = self.stack.pop()?;

                let res = fnearest!(F64, value)?;
                self.stack.push(res);
            }
            Instr::F64Sqrt => {
                let value = self.stack.pop()?;

                let res = fsqrt!(F64, value)?;
                self.stack.push(res);
            }

            Instr::F64Add => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = fadd!(F64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::F64Sub => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = fsub!(F64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::F64Mul => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = fmul!(F64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::F64Div => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = fdiv!(F64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::F64Min => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = fmin!(F64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::F64Max => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = fmax!(F64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::F64Copysign => {
                let (lhs, rhs) = self.stack.pop_pair()?;

                let res = fcopysign!(F64, lhs, rhs)?;
                self.stack.push(res);
            }
            Instr::I32WrapI64 => {
                let value = self.stack.pop()?;

                let res = math::wrap(value)?;
                self.stack.push(res);
            }
            // I32 Trunc
            Instr::I32TruncF32S => {
                let value = self.stack.pop()?;

                let res = trunc!(I32, F32, i32, value)?;
                self.stack.push(res);
            }
            Instr::I32TruncF32U => {
                let value = self.stack.pop()?;

                let res = trunc!(I32, F32, u32, value)?;
                self.stack.push(res);
            }
            Instr::I32TruncF64S => {
                let value = self.stack.pop()?;

                let res = trunc!(I32, F64, i32, value)?;
                self.stack.push(res);
            }
            Instr::I32TruncF64U => {
                let value = self.stack.pop()?;

                let res = trunc!(I32, F64, u32, value)?;
                self.stack.push(res);
            }

            // I64 Extend
            Instr::I64ExtendI32S => {
                let value = self.stack.pop()?;

                let res = math::iextend(value, true)?;
                self.stack.push(res);
            }
            Instr::I64ExtendI32U => {
                let value = self.stack.pop()?;

                let res = math::iextend(value, false)?;
                self.stack.push(res);
            }

            // I64 Trunc
            Instr::I64TruncF32S => {
                let value = self.stack.pop()?;

                let res = trunc!(I64, F32, i32, value)?;
                self.stack.push(res);
            }
            Instr::I64TruncF32U => {
                let value = self.stack.pop()?;

                let res = trunc!(I64, F32, u32, value)?;
                self.stack.push(res);
            }
            Instr::I64TruncF64S => {
                let value = self.stack.pop()?;

                let res = trunc!(I64, F64, i32, value)?;
                self.stack.push(res);
            }
            Instr::I64TruncF64U => {
                let value = self.stack.pop()?;

                let res = trunc!(I64, F64, u32, value)?;
                self.stack.push(res);
            }

            // F32 Convert
            Instr::F32ConvertI32S => {
                let value = self.stack.pop()?;

                let res = convert!(I32 => f32, value, i32)?;
                self.stack.push(res);
            }
            Instr::F32ConvertI32U => {
                let value = self.stack.pop()?;

                let res = convert!(I32 => f32, value)?;
                self.stack.push(res);
            }
            Instr::F32ConvertI64S => {
                let value = self.stack.pop()?;

                let res = convert!(I64 => f32, value, i32)?;
                self.stack.push(res);
            }
            Instr::F32ConvertI64U => {
                let value = self.stack.pop()?;

                let res = convert!(I64 => f32, value)?;
                self.stack.push(res);
            }

            // Demote
            Instr::F32DemoteF64 => {
                let value = self.stack.pop()?;

                let res = math::demote(value)?;
                self.stack.push(res);
            }

            // F64 Convert
            Instr::F64ConvertI32S => {
                let value = self.stack.pop()?;

                let res = convert!(I32 => f64, value, i32)?;
                self.stack.push(res);
            }
            Instr::F64ConvertI32U => {
                let value = self.stack.pop()?;

                let res = convert!(I32 => f64, value)?;
                self.stack.push(res);
            }
            Instr::F64ConvertI64S => {
                let value = self.stack.pop()?;

                let res = convert!(I64 => f64, value, i32)?;
                self.stack.push(res);
            }
            Instr::F64ConvertI64U => {
                let value = self.stack.pop()?;

                let res = convert!(I64 => f64, value)?;
                self.stack.push(res);
            }

            // Promote
            Instr::F64PromoteF32 => {
                let value = self.stack.pop()?;

                let res = math::promote(value)?;
                self.stack.push(res);
            }

            // Reinterp
            Instr::I32ReinterpF32 => {
                let value = self.stack.pop()?;

                let res = reinterp!(I32, value)?;
                self.stack.push(res);
            }
            Instr::F32ReinterpI32 => {
                let value = self.stack.pop()?;

                let res = reinterp!(F32, value)?;
                self.stack.push(res);
            }
            Instr::I64ReinterpF64 => {
                let value = self.stack.pop()?;

                let res = reinterp!(I64, value)?;
                self.stack.push(res);
            }
            Instr::F64ReinterpI64 => {
                let value = self.stack.pop()?;

                let res = reinterp!(F64, value)?;
                self.stack.push(res);
            }
            Instr::Loop(_, _) | Instr::If(_, _, _) | Instr::Block(_, _) | Instr::Else => {
                return Err(Error::InvalidFuncFormat);
            }
            instr => return Err(Error::NotImplemented(instr.clone())),
        };

        Ok(InstrResult::Continue)
    }

    #[inline]
    fn push_frame(&mut self, frame: Frame) {
        self.frames.inner_mut().push(frame);
    }

    #[inline]
    fn get_delta(&mut self, offset: Offset) -> Result<Offset, Error> {
        let delta = get!(I32, self.stack.pop()?)?;

        let new_offset = Offset::from((*offset as i32 + delta) as u32);

        Ok(new_offset)
    }

    fn pop_frame(&mut self) -> Option<Frame> {
        self.frames.inner_mut().pop()
    }

    fn current_frame(&mut self) -> ExecResult<&mut Frame> {
        self.peek_frame_mut().ok_or(Error::EmptyFrameStack)
    }

    fn peek_frame_mut(&mut self) -> Option<&mut Frame> {
        self.frames.inner_mut().last_mut()
    }

    pub fn get_index(expr: Expression) -> Option<usize> {
        for instr in expr.0 {
            match instr {
                Instr::I32Const(val) => return Some(val as usize),
                _ => {}
            }
        }
        None
    }

    pub fn get_constant_init(expr: Expression) -> Result<Value, Error> {
        match &expr.0[0] {
            Instr::I32Const(val) => Ok(Value::from(*val)),
            Instr::I64Const(val) => Ok(Value::from(*val)),
            Instr::F32Const(val) => Ok(Value::from(*val)),
            Instr::F64Const(val) => Ok(Value::from(*val)),
            i => Err(Error::InvalidGlobalInitializer(i.clone())),
        }
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
