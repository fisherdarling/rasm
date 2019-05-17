#[derive(Debug, Clone)]
pub struct Global {}


#[derive(Debug, Copy, Clone)]
pub enum ValueType {
    Int32,
    Int64,
    Float32,
    Float64,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Limit {
    min: u32,
    max: Option<u32>,
}