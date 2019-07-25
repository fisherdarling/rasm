use crate::runtime::interpreter::Interpreter;
use crate::runtime::resolver::{DefaultResolver, Resolver};
use crate::runtime::runtime_store::{DefaultStore, RuntimeStore};
use crate::runtime::ModuleInstance;

use crate::error::{Error, ExecResult};
use crate::index::FuncIdx;
use crate::types::{Value, WasmResult};

#[derive(Debug)]
pub struct Runtime {
    store: Box<dyn RuntimeStore>,
    resolver: Box<dyn Resolver>,
}

impl Runtime {
    pub fn default() -> Runtime {
        Self {
            store: Box::new(DefaultStore::default()),
            resolver: Box::new(DefaultResolver::default()),
        }
    }

    pub fn invoke<N: AsRef<str>, A: AsRef<[Value]>>(
        &mut self,
        name: N,
        args: A,
    ) -> ExecResult<WasmResult> {
        let function_index = self.resolver.resolve_function(None, name.as_ref());
        let function = self.store[function_index].clone().unwrap();
        self.interpreter().execute_function(function, args)
    }

    pub fn invoke_index<A: AsRef<[Value]>>(
        &mut self,
        index: FuncIdx,
        args: A,
    ) -> ExecResult<WasmResult> {
        self.interpreter().execute_function_index(index, args)
    }

    pub fn interpreter(&mut self) -> Interpreter {
        Interpreter::new(&self.resolver, &mut self.store)
    }

    pub fn add_module<Bytes: AsRef<[u8]>>(
        &mut self,
        name: Option<String>,
        bytes: Bytes,
    ) -> ExecResult<()> {
        let builder = ModuleInstance::builder()
            .bytes(&bytes);

        let builder = if let Some(name) = name {
            builder.name(name)
        } else {
            builder
        };


        let instance = builder.instantiate(&mut self.store, &mut self.resolver)?;

        self.store.push_module(instance);
        Ok(())
    }
}
