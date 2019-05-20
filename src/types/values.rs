use crate::types::{
    expression::Expr,   
    GlobalType,
};


#[derive(Debug, Clone)]
pub enum Value {
    Int32(u32),
    Int64(u64),
    Float32(f32),
    Float64(f64),
}

#[derive(Debug, Clone)]
pub struct Global {
    kind: GlobalType, 
    value: Option<Value>,
}

// TODO: Global init