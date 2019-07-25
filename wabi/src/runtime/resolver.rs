use std::collections::HashMap;

use crate::index::{FuncIdx, ModuleIdx};

use crate::function::FuncRef;
use crate::runtime::module_instance::ModuleInstance;

use crate::error::Error;
use std::borrow::Borrow;
use std::fmt;

pub trait Resolver: fmt::Debug {
    fn resolve_module(&self, path: Option<&str>) -> ModuleIdx;

    fn resolve_function(&self, path: Option<&str>, name: &str) -> FuncIdx;

    fn resolve_name(&self, name: &str) -> FuncIdx {
        self.resolve_function(None, name)
    }

    fn insert_module(&mut self, path: Option<&str>, index: ModuleIdx);

    fn insert_function(&mut self, path: Option<&str>, name: &str, index: FuncIdx);
}

#[derive(Debug, Default, Clone)]
pub struct DefaultResolver {
    modules: HashMap<Option<String>, ModuleIdx>,
    functions: HashMap<(Option<String>, String), FuncIdx>,
}

impl Resolver for DefaultResolver {
    fn resolve_module(&self, name: Option<&str>) -> ModuleIdx {
        self.modules[&name.map(Into::into)]
    }

    fn resolve_function(&self, path: Option<&str>, name: &str) -> FuncIdx {
        self.functions[&(path.map(Into::into), name.into())]
    }
    
    fn insert_module(&mut self, path: Option<&str>, index: ModuleIdx) {
        self.modules.insert(path.map(Into::into), index);
    }

    fn insert_function(&mut self, path: Option<&str>, name: &str, index: FuncIdx) {
        self.functions.insert((path.map(Into::into), name.into()), index);
    }
}

// pub struct Resolver {
//     function_names: HashMap<String, FuncIdx>,
//     functions: Vec<FuncRef>,
//     module_names: HashMap<String, ModuleIdx>,
//     modules: Vec<ModuleInstance>,
// }

// impl Resolver {
//     pub fn resolve_module(&self, name: String) -> &ModuleInstance
//     where
//         N: Borrow<str>,
//     {
//         unimplemented!()
//     }

//     pub fn resolve_function(&self, name: String) -> &FuncRef
//     where
//         N: Borrow<str>,
//     {
//         unimplemented!()
//     }

//     pub fn instantiate_from_bytes(bytes: &[u8]) -> Result<ModuleIdx, Error> {
//         let module = wasm_nom::module::ParsedModule::parse_bytes(bytes);

//         unimplemented!()
//     }
// }
