use crate::function::Function;
use crate::module::Module;

use crate::runtime::interpreter::Interpreter;
use crate::store::{Store, StoreBuilder};

use crate::types::index::FuncIdx;
use crate::types::{Data, Global, Value, WasmResult};

use crate::error::{Error, ExecResult};

use wasm_nom::section::export::{Export, ExportDesc};

use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::Path;

#[derive(Debug, Default)]
pub struct ModuleInstance {
    store: Store,
    resolver: HashMap<String, FuncIdx>,
    // stack: Vec<StackElem>,
    // interpreter: Interpreter<'a>,
}

impl ModuleInstance {
    pub fn builder<'a>() -> ModuleInstanceBuilder<'a> {
        ModuleInstanceBuilder::default()
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

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<ModuleInstance, Error> {
        let data = std::fs::read(path)?;

        
        Ok(ModuleInstance::from_bytes(&data)?)
    }

    pub fn from_bytes<A: AsRef<[u8]>>(bytes: A) -> Result<Self, Error> {
        Ok(ModuleInstance::builder().bytes(&bytes).build()?)
    }
}

impl TryFrom<Module> for ModuleInstance {
    type Error = crate::error::Error;

    fn try_from(module: Module) -> Result<ModuleInstance, Self::Error> {
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

        // let store = Store::new(mems, Some(data), functions, globals)?;

        let store = StoreBuilder::default()
            .memory_limit(mems.unwrap_or_default())
            .data(data)
            .functions(funcs)
            .global_inits(globals)
            .build()?;

        // let interpreter = Interpreter::new(&store.functions, &resolver, &mut store.memory);

        Ok(ModuleInstance {
            store,
            resolver,
            // interpreter,
            // stack: Vec::with_capacity(256),
        })
    }
}

#[derive(Default)]
pub struct ModuleInstanceBuilder<'a> {
    bytes: Option<&'a dyn AsRef<[u8]>>,
    module: Option<Module>,
    store: Option<Store>,
    functions: Option<Vec<Function>>,
    data: Option<Data>,
    exports: Option<Export>,
    global_inits: Option<Vec<Global>>,
    resolver: Option<HashMap<String, FuncIdx>>,
}

impl<'a> ModuleInstanceBuilder<'a> {
    #[inline]
    pub fn bytes(self, bytes: &'a impl AsRef<[u8]>) -> Self {
        Self {
            bytes: Some(bytes),
            ..self
        }
    }

    #[inline]
    pub fn module(self, module: Module) -> Self {
        Self {
            module: Some(module),
            ..self
        }
    }

    #[inline]
    pub fn store(self, store: Store) -> Self {
        Self {
            store: Some(store),
            ..self
        }
    }

    #[inline]
    pub fn functions(self, functions: Vec<Function>) -> Self {
        Self {
            functions: Some(functions),
            ..self
        }
    }

    #[inline]
    pub fn data(self, data: Data) -> Self {
        Self {
            data: Some(data),
            ..self
        }
    }

    #[inline]
    pub fn exports(self, exports: Export) -> Self {
        Self {
            exports: Some(exports),
            ..self
        }
    }

    #[inline]
    pub fn global_inits(self, global_inits: Vec<Global>) -> Self {
        Self {
            global_inits: Some(global_inits),
            ..self
        }
    }

    #[inline]
    pub fn resolver(self, resolver: HashMap<String, FuncIdx>) -> Self {
        Self {
            resolver: Some(resolver),
            ..self
        }
    }

    pub fn build(self) -> Result<ModuleInstance, Error> {
        if let Some(module) = self.module {
            return ModuleInstance::try_from(module);
        } else if let Some(bytes) = self.bytes {
            let module = Module::from_bytes(bytes.as_ref());

            return ModuleInstance::try_from(module);
        }

        unimplemented!()
    }
}
