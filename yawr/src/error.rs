use crate::types::index::{LabelIdx, FuncIdx};
use crate::types::{ValType, Value};

use failure::Fail;

pub type ExecResult<T> = Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Invalid Operand Type")]
    InvalidOperand,
    #[fail(display = "Type mismatch: {}", 0)]
    TypeMismatch(u32),
    #[fail(display = "The Number of arguments is different than what the signature requires.")]
    FunctionArgumentCount,
    #[fail(display = "Invalid argument types. Expected: {:?}, Got: {:?}", 0, 1)]
    FunctionArgumentTypes(ValType, Value),
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
    #[fail(display = "Empty Frame Value Stack")]
    EmptyValueStack,
    #[fail(display = "Branch depth too deep: {:?}", 0)]
    BranchDepth(LabelIdx),
    #[fail(display = "Cannot unpause a not paused Frame")]
    UnpauseFrame,
    #[fail(display = "Undefined Float")]
    UndefinedFloat
}
