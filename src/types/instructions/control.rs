use crate::types::ResType;
use crate::types::index::{LabelIdx, TypeIdx, FuncIdx};
use crate::types::instructions::Instr as RealInstr;

#[derive(Debug, Clone, PartialEq)]
pub enum Instr {
    Nop,
    Unreachable,
    Block(ResType, Vec<RealInstr>),
    Loop(ResType, Vec<RealInstr>),
    If(ResType, Vec<RealInstr>, Vec<RealInstr>),
    Br(LabelIdx),
    BrIf(LabelIdx),
    BrTable(Vec<LabelIdx>, LabelIdx),
    Return,
    Call(FuncIdx),
    CallIndirect(TypeIdx),
}