use crate::function::Function;
use crate::module::Module;
use crate::store::Store;
use crate::types::index::FuncIdx;
use crate::types::{WasmResult, WasmValue};
use wasamere::section::export::{Export, ExportDesc};

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Runtime {
    store: Store,
    resolver: HashMap<String, FuncIdx>,
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
        }
    }

    pub fn invoke(&self, name: String, args: Vec<WasmValue>) -> WasmResult {
        WasmResult::Unit
    }

    // #[derive(Debug, Clone, PartialEq)]
    // pub struct Export {
    //     name: String,
    //     desc: ExportDesc,
    // }
}
