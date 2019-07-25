use std::ops::{Index, IndexMut};

use crate::function::FuncRef;
use crate::index::{FuncIdx, ModuleIdx};
use crate::runtime::ModuleInstance;

pub trait RuntimeStore:
    IndexMut<ModuleIdx, Output = ModuleInstance> + Index<FuncIdx, Output = Option<FuncRef>>
{
}

#[derive(Default, Debug, Clone)]
pub struct DefaultStore {
    modules: Vec<ModuleInstance>,
    functions: Vec<Option<FuncRef>>,
}

impl DefaultStore {
    pub fn modules_len(&self) -> usize {
        self.modules.len()
    }

    pub fn functions_len(&self) -> usize {
        self.functions.len()
    }

    pub fn push_module(&mut self, instance: ModuleInstance) -> ModuleIdx {
        self.modules.push(instance);
        (self.modules.len() - 1).into()
    }

    pub fn push_function(&mut self, function: FuncRef) -> FuncIdx {
        self.functions.push(Some(function));
        (self.functions.len() - 1).into()
    }

    pub fn push_empty_function(&mut self) -> FuncIdx {
        self.functions.push(None);
        (self.functions.len() - 1).into()
    
    }
}

impl RuntimeStore for DefaultStore {}

impl Index<ModuleIdx> for DefaultStore {
    type Output = ModuleInstance;

    fn index(&self, index: ModuleIdx) -> &Self::Output {
        &self.modules[index.as_usize()]
    }
}

impl IndexMut<ModuleIdx> for DefaultStore {
    fn index_mut(&mut self, index: ModuleIdx) -> &mut Self::Output {
        &mut self.modules[index.as_usize()]
    }
}

impl Index<FuncIdx> for DefaultStore {
    type Output = Option<FuncRef>;

    fn index(&self, index: FuncIdx) -> &Self::Output {
        &self.functions[index.as_usize()]
    }
}
