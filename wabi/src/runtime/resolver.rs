use std::collections::HashMap;

use crate::index::{FuncIdx, ModuleIdx};

use crate::function::FuncRef;
use crate::runtime::module_instance::ModuleInstance;

use crate::error::Error;
use std::borrow::Borrow;

pub struct Resolver {
    function_names: HashMap<String, FuncIdx>,
    functions: Vec<FuncRef>,
    module_names: HashMap<String, ModuleIdx>,
    modules: Vec<ModuleInstance>,
}

impl Resolver {
    pub fn resolve_module<N>(&self, name: N) -> &ModuleInstance
    where
        N: Borrow<str>,
    {
        unimplemented!()
    }

    pub fn resolve_function<N>(&self, name: N) -> &FuncRef
    where
        N: Borrow<str>,
    {
        unimplemented!()
    }

    pub fn instantiate_from_bytes(bytes: &[u8]) -> Result<ModuleIdx, Error> {
        let module = wasm_nom::module::ParsedModule::parse_bytes(bytes);
        
        

        unimplemented!()
    }
}
