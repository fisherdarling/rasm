use std::collections::HashMap;
use std::ops::{Index, IndexMut};

use crate::function::Function;
use crate::types::index::FuncIdx;

#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    functions: HashMap<FuncIdx, Function>,
}

impl Store {
    pub fn new(functions: HashMap<FuncIdx, Function>) -> Self {
        Self { functions }
    }

    pub fn from_functions(functions: Vec<Function>) -> Self {
        let map: HashMap<FuncIdx, Function> = functions
            .into_iter()
            .enumerate()
            .map(|(i, f)| (FuncIdx::from(i as u32), f))
            .collect();

        Self { functions: map }
    }
}

impl<'a> Index<&'a FuncIdx> for Store {
    type Output = Function;

    fn index(&self, func: &'a FuncIdx) -> &Self::Output {
        &self.functions[func]
    }
}
