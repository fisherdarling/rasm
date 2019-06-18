use crate::types::index::{FuncIdx, GlobalIdx, LabelIdx};
use crate::types::{ValType, Value};

use std::collections::CollectionAllocErr;

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
    #[fail(display = "Invalid Function Format. Functions must be `flattened` before execution.")]
    InvalidFuncFormat,
    #[fail(display = "Empty Frame Stack")]
    EmptyFrameStack,
    #[fail(display = "Empty Frame Value Stack")]
    EmptyValueStack,
    #[fail(display = "Branch depth too deep: {:?}", 0)]
    BranchDepth(LabelIdx),
    #[fail(display = "Cannot unpause a not paused Frame")]
    UnpauseFrame,
    #[fail(display = "Undefined Float")]
    UndefinedFloat,
    #[fail(display = "Memory offset must be as single `(i32.const offset)` instruction.")]
    OffsetExpression,
    #[fail(display = "Encountered an error growing the memory: {:?}", 0)]
    MemoryGrow(CollectionAllocErr),
    #[fail(display = "The maximum amount of memory was exceeded")]
    MemoryExceeded,
    #[fail(display = "Invalid global index {:?}", 0)]
    GlobalIndex(GlobalIdx),
    #[fail(display = "Global cannot be modified: {:?}", 0)]
    GlobalMut(GlobalIdx),
    #[fail(display = "IOError: {:?}", 0)]
    IOError(std::io::Error),
    #[fail(display = "Invalid Global Initializer: {:?}", 0)]
    InvalidGlobalInitializer(wasamere::instr::Instr),
    #[fail(display = "Unreachable Trap")]
    TrapUnreachable,
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IOError(error)
    }
}
