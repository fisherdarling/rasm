use crate::types::Value;
use wasamere::types::{Mut, ValType};

pub struct GlobalInst {
    ty: ValType,
    var: Mut,
    value: Value,
}

impl GlobalInst {
    pub fn new(ty: ValType, var: Mut, value: Value) -> Self {
        GlobalInst { ty, var, value }
    }

    pub fn default(ty: ValType, var: Mut) -> Self {
        let value = Value::default_valtype(ty);
        GlobalInst { ty, var, value }
    }
}
