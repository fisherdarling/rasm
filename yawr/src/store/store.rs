use std::collections::HashMap;
use std::ops::Index;

use crate::function::{FuncRef, Function};
use crate::store::memory::MemInst;
use crate::types::{index::FuncIdx, Data, Limit};

#[derive(Debug, Clone)]
pub struct Store {
    pub(crate) functions: Vec<FuncRef>,
    pub(crate) memory: MemInst,
}

impl Store {
    pub fn new(
        mems: Option<Limit>,
        data: Option<Vec<Data>>,
        functions: Vec<FuncRef>,
    ) -> Self {
        let (min, max) = if let Some(Limit { min, max }) = mems {
            (min, max)
        } else {
            (0, None)
        };

        let mut memory = MemInst::new(min, max);
        memory.init(data);

        Self { functions, memory }
    }

    pub fn new_with_functions(
        mems: Option<Limit>,
        data: Option<Vec<Data>>,
        functions: Vec<Function>,
    ) -> Self {
        let (min, max) = if let Some(Limit { min, max }) = mems {
            (min, max)
        } else {
            (0, None)
        };

        let mut memory = MemInst::new(min, max);
        memory.init(data);

        let functions: Vec<FuncRef> = functions.into_iter().map(|v| FuncRef::new(v)).collect();

        Self {
            functions,
            memory,
        }
    }
}

impl<'a> Index<&'a FuncIdx> for Store {
    type Output = Function;

    fn index(&self, func: &'a FuncIdx) -> &Self::Output {
        &self.functions[func.as_usize()]
    }
}
