use crate::types::index::{GlobalIdx, LocalIdx};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instr {
    LocalGet(LocalIdx),
    LocalSet(LocalIdx),
    LocalTee(LocalIdx),
    GlobalGet(GlobalIdx),
    GlobalSet(GlobalIdx),
}