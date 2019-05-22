use crate::types::instructions::numeric::{Bitlen, Signed};

pub type MemArg = (u32, u32);

pub enum Memop {
    Load(Bitlen, MemArg),
    Store(Bitlen, MemArg),
    Load8(Bitlen, Signed, MemArg),
    Store8(Bitlen, Memarg),
    Load16(Bitlen, Signed, MemArg),
    Store16(Bitlen, Memarg),
    I64_Load32(Signed, Memarg),
    I64_Store32(MemArg),
    Size,
    Grow,
}