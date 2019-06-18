use crate::function::FuncRef;
use crate::function::Function;
use crate::module::Module;
use crate::runtime::frame::{Frame, StackElem};
use crate::runtime::interpreter::Interpreter;
use crate::store::Store;

use crate::types::index::{FuncIdx, LocalIdx};
use crate::types::{ResType, ValType, Value, WasmResult};

use crate::error::{Error, ExecResult};

use wasamere::instr::Instr;
use wasamere::section::export::{Export, ExportDesc};

use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
pub struct Runtime {
    store: Store,
    resolver: HashMap<String, FuncIdx>,
    // stack: Vec<StackElem>,
    // interpreter: Interpreter<'a>,
}

impl Runtime {
    pub fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Runtime, Error> {
        let module = Module::from_bytes(bytes.as_ref());

        let funcs: Vec<Function> = module.funcs;
        let exports: Vec<Export> = module.exports;
        let mems = module.mems;
        let data = module.data;
        let globals = module.globals;

        // debug!("Mems: {}")

        let mut resolver: HashMap<String, FuncIdx> = HashMap::new();

        for Export { name, desc } in exports {
            match desc {
                ExportDesc::Func(idx) => {
                    resolver.insert(name, FuncIdx::from(idx.index()));
                }
                _ => {}
            }
        }

        let functions = funcs.into_iter().map(FuncRef::new).collect();

        let store = Store::new(mems, Some(data), functions, globals)?;
        // let interpreter = Interpreter::new(&store.functions, &resolver, &mut store.memory);

        Ok(Runtime {
            store,
            resolver,
            // interpreter,
            // stack: Vec::with_capacity(256),
        })
    }

    pub fn invoke<N: Into<String>, A: AsRef<[Value]>>(
        &mut self,
        name: N,
        args: A,
    ) -> ExecResult<WasmResult> {
        self.interpreter().invoke(name, args)
    }

    pub fn invoke_index<A: AsRef<[Value]>>(
        &mut self,
        idx: usize,
        args: A,
    ) -> ExecResult<WasmResult> {
        self.interpreter().invoke_index(idx, args)
    }

    pub fn interpreter(&mut self) -> Interpreter {
        Interpreter::new(&self.resolver, &mut self.store)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Runtime, Error> {
        let data = std::fs::read(path)?;

        Ok(Runtime::from_bytes(&data)?)
    }
}
