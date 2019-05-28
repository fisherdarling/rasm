// use std::collections::HashMap;

use crate::module::Module;
use crate::types::Function as PFunction;
use crate::types::{Locals, ResType, ValType};

use std::rc::Rc;

use wasamere::instr::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Signature {
    params: Vec<ValType>,
    result: ResType,
}

impl From<crate::types::FuncType> for Signature {
    fn from(func: crate::types::FuncType) -> Self {
        Self {
            params: func.0,
            result: func.1,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub signature: Signature,
    pub locals: Locals,
    pub body: Expression,
}

impl Function {
    pub fn new(signature: Signature, locals: Locals, body: Expression) -> Self {
        Self {
            signature,
            locals,
            body,
        }
    }
}
