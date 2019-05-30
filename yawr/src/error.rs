use crate::types::index::FuncIdx;

use failure::Fail;

pub type ExecResult<T> = Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Invalid Operand Type")]
    InvalidOperand,
    #[fail(display = "Type mismatch")]
    TypeMismatch,
    #[fail(display = "Instruction not implemented: {:?}", 0)]
    NotImplemented(wasamere::instr::Instr),
    #[fail(display = "Corrupted Value Stack")]
    ValueStack,
    #[fail(display = "Invalid Function Name")]
    InvalidFunctionName(String),
    #[fail(display = "Invalid Function Index: {:?}", 0)]
    InvalidFuncIdx(FuncIdx),
    #[fail(display = "Empty Frame Stack")]
    EmptyFrameStack,
}