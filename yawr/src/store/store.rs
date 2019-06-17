use std::collections::HashMap;
use std::ops::Index;

use crate::function::{Function, FuncRef};
use crate::store::memory::MemInst;
use crate::types::{Limit, Data, index::FuncIdx};

#[derive(Debug, Clone)]
pub struct Store {
    pub(crate) functions: HashMap<FuncIdx, FuncRef>,
    pub(crate) memory: MemInst,
}

impl Store {
    pub fn new(mems: Option<Limit>, data: Option<Vec<Data>>, functions: HashMap<FuncIdx, FuncRef>) -> Self {
        let (min, max) = if let Some(Limit { min, max }) = mems {
            (min, max)
        } else {
            (0, None)       
        };

        let mut memory = MemInst::new(min, max);
        memory.init(data);
        
        Self { functions, memory }
    }

    pub fn new_with_functions(mems: Option<Limit>, data: Option<Vec<Data>>, functions: Vec<Function>) -> Self {
        let (min, max) = if let Some(Limit { min, max }) = mems {
            (min, max)
        } else {
            (0, None)       
        };

        let mut memory = MemInst::new(min, max);
        memory.init(data);
        
        let map: HashMap<FuncIdx, FuncRef> = functions
            .into_iter()
            .enumerate()
            .map(|(i, f)| (FuncIdx::from(i as u32), FuncRef::new(f)))
            .collect();

        Self { functions: map, memory }
    }
}

impl<'a> Index<&'a FuncIdx> for Store {
    type Output = Function;

    fn index(&self, func: &'a FuncIdx) -> &Self::Output {
        &self.functions[func]
    }
}