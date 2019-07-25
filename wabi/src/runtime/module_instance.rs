use crate::function::{Function, Signature, FuncRef};
use crate::module::Module;

use crate::runtime::resolver::Resolver;
use crate::runtime::runtime_store::RuntimeStore;

use crate::runtime::interpreter::Interpreter;
use crate::store::{Store, StoreBuilder};

use crate::types::index::FuncIdx;
use crate::types::{Data, Global, Value, WasmResult, Locals};

use crate::error::{Error, ExecResult};

use crate::instr::Expression;
use wasm_nom::section::export::{Export, ExportDesc};
use wasm_nom::section::import::{Import, ImportDesc};

use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::Path;

#[derive(Debug, Default, Clone)]
pub struct ModuleInstance {
    store: Store,
    // resolver: HashMap<String, FuncIdx>,
    // stack: Vec<StackElem>,
    // interpreter: Interpreter<'a>,
}

impl ModuleInstance {
    pub fn builder<'a>() -> ModuleInstanceBuilder<'a> {
        ModuleInstanceBuilder::default()
    }

    // pub fn invoke<N: Into<String>, A: AsRef<[Value]>>(
    //     &mut self,
    //     name: N,
    //     args: A,
    // ) -> ExecResult<WasmResult> {
    //     self.interpreter().invoke(name, args)
    // }

    // pub fn invoke_index<A: AsRef<[Value]>>(
    //     &mut self,
    //     idx: usize,
    //     args: A,
    // ) -> ExecResult<WasmResult> {
    //     self.interpreter().invoke_index(idx, args)
    // }

    // pub fn interpreter(&mut self) -> Interpreter {
    //     Interpreter::new(&self.resolver, &mut self.store)
    // }

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
        // let funcs: Vec<Function> = module.funcs;
        let funcs: Vec<Function> = unimplemented!();
        let exports: Vec<Export> = module.exports;
        let mems = module.mems;
        let data = module.data;
        let globals = module.globals;

        // debug!("Mems: {}")

        let mut resolver: HashMap<String, FuncIdx> = HashMap::new();

        // for Import { module, name, desc } in imports {
            
        // }

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
    exports: Option<Vec<Export>>,
    imports: Option<Vec<Import>>,
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
    pub fn exports(self, exports: Vec<Export>) -> Self {
        Self {
            exports: Some(exports),
            ..self
        }
    }

    #[inline]
    pub fn imports(self, imports: Vec<Import>) -> Self {
        Self {
            imports: Some(imports),
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

    pub fn instantiate(
        self,
        store: &mut impl RuntimeStore,
        resolver: &mut impl Resolver,
    ) -> Result<ModuleInstance, Error> {
        // let funcs: Vec<Function> = module.funcs;
        let module = if let Some(bytes) = self.bytes {
            Module::from_bytes(bytes.as_ref())
        } else {
            panic!()
        };

        let funcs: Vec<(Signature, Locals, Expression)> = module.funcs;
        let func_indicies: Vec<FuncIdx> = Vec::new();

        for (signature, locals, expr) in funcs {
            let function = Function::new(signature, locals, expr, store.modules_len().into());
            let function = FuncRef::new(function);

            let mut idx = store.push_function(function);
            func_indicies.push(idx);
        }

        let exports: Vec<Export> = module.exports;

        for Export { name, desc } in exports {
            match desc {
                ExportDesc::Func(func_idx) => {
                    let new_idx = func_indicies[func_idx.as_usize()];
                    resolver.insert_function(None, &name, new_idx);
                }
            }
        }

        let imports: Vec<Import> = module.imports;
        let mut imported_funcs: Vec<FuncIdx> = Vec::new(); 
        for Import { module, name, desc } in imports {
            match desc {
                ImportDesc::Func(_) => {
                    let idx = resolver.resolve_function(Some(&module), &name);
                    imported_funcs.push(idx);
                },
                _ => {}
            }
        }



        let imports: Vec<Import> = module.imports;
        let mems = module.mems;
        let data = module.data;
        let globals = module.globals;

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
            // interpreter,
            // stack: Vec::with_capacity(256),
        })
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
