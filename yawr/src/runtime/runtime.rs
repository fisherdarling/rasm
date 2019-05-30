use crate::function::Function;
use crate::module::Module;
use crate::runtime::frame::{Frame, StackElem};
use crate::runtime::interpreter::Interpreter;
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

    pub fn invoke<N: Into<String>, A: AsRef<[Value]>>(&mut self, name: N, args: A) -> ExecResult<WasmResult> {
        let mut runner = Interpreter::new(self.store.functions.clone(), self.resolver.clone());

        runner.invoke(name, args)
    }
}


