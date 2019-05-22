use crate::types::index::{FuncIdx, LabelIdx, TypeIdx};
use crate::types::instructions::Instr as RealInstr;
use crate::types::ResType;

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
