use crate::types::ResType;
use crate::types::index::{LabelIdx, TypeIdx, FuncIdx};
use crate::types::instructions::Instr;

pub enum CtrlOp {
    Nop,
    Unreachable,
    Block(ResType, Vec<Instr>),
    Loop(ResType, Vec<Instr>),
    If(ResType, Vec<Instr>, Vec<Instr>),
    Br(LabelIdx),
    BrIf(LabelIdx),
    BrTable(Vec<LabelIdx>, LabelIdx),
    Return,
    Call(FuncIdx),
    CallIndirect(TypeIdx),
}