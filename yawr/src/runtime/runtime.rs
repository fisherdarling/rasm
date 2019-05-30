use crate::function::Function;
use crate::module::Module;
use crate::runtime::frame::{Frame, StackElem};
use crate::store::Store;

use crate::types::index::{FuncIdx, LocalIdx};
use crate::types::{ValType, WasmResult, Value, ResType};

use crate::error::{Error, ExecResult};

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

    pub fn invoke(&mut self, name: String, args: Vec<Value>) -> ExecResult<WasmResult> {
        let idx = self.resolver[&name];
        let function = &self.store[&idx];

        let frame = function.instantiate(&args);

        // let frame = Frame::new(locals);
        // let code: &[Instr] = &function.body.0;

        // self.execute_with_frame(frame)

        Ok(WasmResult::Unit)
    }

    pub fn execute(&self, frame: Frame, code: &[Instr], restype: ResType) -> ExecResult<WasmResult>  {
        
        let mut stack: Vec<StackElem> = Vec::new();

        for instr in code {
            match instr {
                Instr::LocalGet(idx) => 
                stack
                    .push(StackElem::Value(frame.locals[idx.index() as usize])),
                Instr::I32Add => {
                    let op_2 = stack.pop().unwrap();
                    let op_1 = stack.pop().unwrap();

                    if let (StackElem::Value(a), StackElem::Value(b)) = (op_1, op_2) {
                        let res = StackElem::Value((a + b)?);

                        stack.push(res);
                    }
                }
                // Instr::I32Sub {



                // }
                _ => return Err(Error::NotImplemented(instr.clone())),
            }
        }

        match restype {
            ResType::Unit => {
                assert!(self.stack.is_empty());
                Ok(WasmResult::Unit)
            },
            ResType::ValType(ValType::I32) => {
                if let StackElem::Value(Value::I32(res)) = stack.pop().unwrap() {
                    Ok(WasmResult::I32(res))
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


