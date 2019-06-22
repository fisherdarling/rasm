
use std::ops::Index;

use crate::function::{FuncRef, Function};
use crate::store::global::GlobalInst;
use crate::store::memory::MemInst;
// use crate::store::table::TableInst;
use crate::types::{index::FuncIdx, Data, Global, Limit};

use crate::error::Error;

#[derive(Debug, Clone, Default)]
pub struct Store {
    pub(crate) functions: Vec<FuncRef>,
    pub(crate) memory: MemInst,
    pub(crate) globals: Vec<GlobalInst>,
    // pub(crate) table: TableInst,
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
        memory.init(data)?;

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

    pub fn new_functions(
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
        memory.init(data)?;

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

#[derive(Debug, Default, Clone)]
pub struct StoreBuilder {
    data: Option<Vec<Data>>,
    mems: Option<Limit>,
    func_refs: Option<Vec<FuncRef>>,
    functions: Option<Vec<Function>>,
    globals: Option<Vec<GlobalInst>>,
    global_inits: Option<Vec<Global>>,
}

impl StoreBuilder {
    pub fn data(self, data: Vec<Data>) -> Self {
        Self {
            data: Some(data),
            ..self
        }
    }

    pub fn memory_limit(self, mems: Limit) -> Self {
        Self {
            mems: Some(mems),
            ..self
        }
    }

    pub fn func_refs(self, func_refs: Vec<FuncRef>) -> Self {
        Self {
            func_refs: Some(func_refs),
            ..self
        }
    }

    pub fn functions(self, functions: Vec<Function>) -> Self {
        Self {
            functions: Some(functions),
            ..self
        }
    }

    pub fn global_inits(self, global_inits: Vec<Global>) -> Self {
        Self {
            global_inits: Some(global_inits),
            ..self
        }
    }

    pub fn global_instances(self, globals: Vec<GlobalInst>) -> Self {
        Self {
            globals: Some(globals),
            ..self
        }
    }

    pub fn build(self) -> Result<Store, Error> {
        let (min, max) = if let Some(Limit { min, max }) = self.mems {
            (min, max)
        } else {
            (0, None)
        };

        let mut memory = MemInst::new(min, max);
        memory.init(self.data)?;

        let globals = if let Some(globals) = self.globals {
            globals
        } else if let Some(global_inits) = self.global_inits {
            global_inits
                .into_iter()
                .map(GlobalInst::from_global)
                .collect::<Result<Vec<GlobalInst>, Error>>()?
        } else {
            Vec::new()
        };

        let functions = if let Some(functions) = self.functions {
            functions.into_iter().map(|v| FuncRef::new(v)).collect()
        } else if let Some(func_refs) = self.func_refs {
            func_refs
        } else {
            Vec::new()
        };

        Ok(Store {
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

mod error {}
