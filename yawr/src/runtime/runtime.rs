use crate::function::Function;
use crate::module::Module;
use crate::runtime::frame::{Frame, StackElem};
use crate::runtime::interpreter::Interpreter;
// use crate::store_old::Store;
use crate::store::Store;

use crate::types::index::{FuncIdx, LocalIdx};
use crate::types::{ResType, ValType, Value, WasmResult};

use crate::error::{Error, ExecResult};

use wasamere::instr::Instr;
use wasamere::section::export::{Export, ExportDesc};

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Runtime {
    store: Store,
    resolver: HashMap<String, FuncIdx>,
    stack: Vec<StackElem>,
    interpreter: Interpreter,
}

impl Runtime {
    pub fn from_bytes(bytes: &[u8]) -> Runtime {
        let module = Module::from_bytes(bytes);

        let funcs: Vec<Function> = module.funcs;
        let exports: Vec<Export> = module.exports;
        let mems = module.mems;
        let data = module.data;

        // debug!("Mems: {}")
    
        let mut resolver: HashMap<String, FuncIdx> = HashMap::new();

        for Export { name, desc } in exports {
            match desc {
                ExportDesc::Func(idx) => {
                    resolver.insert(name, FuncIdx::from(idx.index()));
                }
                _ => {},
            }
        }


        let store = Store::new_with_functions(mems, Some(data), funcs);
        let interpreter = Interpreter::new(store.functions.clone(), resolver.clone(), store.clone());

        Runtime {
            store,
            resolver,
            interpreter,
            stack: Vec::with_capacity(256),
        }
    }

    pub fn invoke<N: Into<String>, A: AsRef<[Value]>>(
        &mut self,
        name: N,
        args: A,
    ) -> ExecResult<WasmResult> {
        self.interpreter.invoke(name, args)
    }
}
