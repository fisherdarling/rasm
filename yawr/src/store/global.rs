use crate::types::Value;
use wasamere::types::{Global};
use wasamere::types::{Mut, ValType};

use crate::error::Error;

#[derive(Debug, Clone, Copy)]
pub struct GlobalInst {
    pub ty: ValType,
    pub var: Mut,
    pub value: Value,
}

impl GlobalInst {
    pub fn new(ty: ValType, var: Mut, value: Value) -> Self {
        GlobalInst { ty, var, value }
    }

    pub fn set(&mut self, value: Value) {
        self.value = value;
    }

    pub fn get(&self) -> Value {
        self.value
    }

    pub fn default(ty: ValType, var: Mut) -> Self {
        let value = Value::default_valtype(ty);
        GlobalInst { ty, var, value }
    }

    pub fn from_global(global: Global) -> Result<GlobalInst, Error> {
        use crate::runtime::interpreter::Interpreter;

        let globaltype = global.0;
        let ty = globaltype.0;
        let var = globaltype.1;
        let init_expr = Interpreter::get_constant_init(global.1)?;

        Ok(GlobalInst::new(ty, var, init_expr))
    }
}
