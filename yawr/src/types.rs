pub use wasamere::types::*;
// pub use wasamere::

#[derive(Debug, Clone, Copy)]
pub enum WasmValue {
    I32(u32),
    I64(u64),
    F32(f32),
    F64(f64),
}

#[derive(Debug, Clone, Copy)]
pub enum WasmResult {
    I32(u32),
    I64(u64),
    F32(f32),
    F64(f64),
    Unit,
}
