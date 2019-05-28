use crate::function::Function;
use crate::module::Module;
use crate::runtime::frame::{Frame, StackElem};
use crate::store::Store;

use crate::types::index::{FuncIdx, LocalIdx};
use crate::types::{ValType, WasmResult, WasmValue, ResType};

use wasamere::instr::Instr;
use wasamere::section::export::{Export, ExportDesc};

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Runtime {
    store: Store,
    resolver: HashMap<String, FuncIdx>,
    stack: Vec<StackElem>,
}

impl Runtime {
    pub fn from_bytes(bytes: &[u8]) -> Runtime {
        let module = Module::from_bytes(bytes);

        let funcs: Vec<Function> = module.funcs;
        let exports: Vec<Export> = module.exports;

        let mut resolver: HashMap<String, FuncIdx> = HashMap::new();

        for Export { name, desc } in exports {
            match desc {
                ExportDesc::Func(idx) => {
                    resolver.insert(name, FuncIdx::from(idx.index()));
                }
                _ => panic!(),
            }
        }

        let store = Store::from_functions(funcs);

        Runtime {
            store,
            resolver,
            stack: Vec::with_capacity(256),
        }
    }

    pub fn invoke(&mut self, name: String, args: Vec<WasmValue>) -> WasmResult {
        let idx = self.resolver[&name];
        let function = &self.store[&idx];

        let mut locals: Vec<WasmValue> = Vec::new();

        for (arg, param) in args.into_iter().zip(function.signature.params.iter()) {
            match (arg, param) {
                (a @ WasmValue::I32(_), ValType::I32) => locals.push(a),
                (a @ WasmValue::I64(_), ValType::I64) => locals.push(a),
                (a @ WasmValue::F32(_), ValType::F32) => locals.push(a),
                (a @ WasmValue::F64(_), ValType::F64) => locals.push(a),
                _ => panic!("Invalid argument types"),
            }
        }

        for local in &function.locals.0 {
            match local {
                ValType::I32 => locals.push(WasmValue::I32(0)),
                ValType::I64 => locals.push(WasmValue::I64(0)),
                ValType::F32 => locals.push(WasmValue::F32(0.0)),
                ValType::F64 => locals.push(WasmValue::F64(0.0)),
            }
        }

        let frame = Frame::new(locals);
        let code: &[Instr] = &function.body.0;

        self.execute(frame, code, function.signature.result.clone())
    }

    pub fn execute(&self, frame: Frame, code: &[Instr], restype: ResType) -> WasmResult {
        
        let mut stack: Vec<StackElem> = Vec::new();

        for instr in code {
            match instr {
                Instr::LocalGet(idx) => 
                stack
                    .push(StackElem::Value(frame.locals[idx.index() as usize])),
                Instr::I32Add => {
                    let op_2 = stack.pop().unwrap();
                    let op_1 = stack.pop().unwrap();

                    match (op_1, op_2) {
                        (
                            StackElem::Value(WasmValue::I32(a)),
                            StackElem::Value(WasmValue::I32(b)),
                        ) => stack.push(StackElem::Value(WasmValue::I32(a + b))),
                        _ => panic!(),
                    }
                }
                _ => unimplemented!("Instruction not yet implemented: {:?}", instr),
            }
        }

        match restype {
            ResType::Unit => {
                assert!(self.stack.is_empty());
                WasmResult::Unit
            },
            ResType::ValType(ValType::I32) => {
                if let StackElem::Value(WasmValue::I32(res)) = stack.pop().unwrap() {
                    WasmResult::I32(res)
                } else {
                    panic!()
                }
            },
            // ResType::ValType(ValType::I64) => ,
            // ResType::ValType(ValType::F32) => ,
            // ResType::ValType(ValType::F64) => ,
            _ => panic!()
        }

        // return self.stack.pop().unwrap()
    }

    //     WasmResult::Unit
    // }

    // #[derive(Debug, Clone, PartialEq)]
    // pub struct Export {
    //     name: String,
    //     desc: ExportDesc,
    // }
}
