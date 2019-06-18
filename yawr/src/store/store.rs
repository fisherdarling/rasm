use std::collections::HashMap;
use std::ops::Index;

use crate::function::{FuncRef, Function};
use crate::store::global::GlobalInst;
use crate::store::memory::MemInst;
use crate::types::{index::FuncIdx, Data, Global, Limit};

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct Store {
    pub(crate) functions: Vec<FuncRef>,
    pub(crate) memory: MemInst,
    pub(crate) globals: Vec<GlobalInst>,
}

impl Store {
    pub fn new(
        mems: Option<Limit>,
        data: Option<Vec<Data>>,
        functions: Vec<FuncRef>,
        globals: Vec<Global>,
    ) -> Result<Self, Error> {
        let (min, max) = if let Some(Limit { min, max }) = mems {
            (min, max)
        } else {
            (0, None)
        };

        let mut memory = MemInst::new(min, max);
        memory.init(data);

        let globals: Vec<GlobalInst> = globals
            .into_iter()
            .map(GlobalInst::from_global)
            .collect::<Result<Vec<GlobalInst>, Error>>()?;

        Ok(Self {
            functions,
            memory,
            globals,
        })
    }

    pub fn new_with_functions(
        mems: Option<Limit>,
        data: Option<Vec<Data>>,
        functions: Vec<Function>,
        globals: Vec<Global>,
    ) -> Result<Self, Error> {
        let (min, max) = if let Some(Limit { min, max }) = mems {
            (min, max)
        } else {
            (0, None)
        };

        let mut memory = MemInst::new(min, max);
        memory.init(data);

        let globals: Vec<GlobalInst> = globals
            .into_iter()
            .map(GlobalInst::from_global)
            .collect::<Result<Vec<GlobalInst>, Error>>()?;

        let functions: Vec<FuncRef> = functions.into_iter().map(|v| FuncRef::new(v)).collect();

        Ok(Self {
            functions,
            memory,
            globals,
        })
    }
}

impl<'a> Index<&'a FuncIdx> for Store {
    type Output = Function;

    fn index(&self, func: &'a FuncIdx) -> &Self::Output {
        &self.functions[func.as_usize()]
    }
}
