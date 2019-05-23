use nom::take;
use nom::ErrorKind;
use nom::IResult;

use crate::types::index::{FuncIdx, LabelIdx, LocalIdx, TypeIdx};
use crate::types::ResType;

pub type MemArg = (u32, u32);

#[derive(Debug, Clone, PartialEq)]
pub enum Instr {
    // Ast Only:
    End,
    Else,

    // Control Instructions:
    Unreachable,
    Nop,
    Block(ResType, Vec<Instr>),
    Loop(ResType, Vec<Instr>),
    If(ResType, Vec<Instr>, Vec<Instr>),
    Br(LabelIdx),
    BrIf(LabelIdx),
    // Figure out meaning of l_N
    BrTable(Vec<LabelIdx>, LabelIdx),
    Return,
    Call(FuncIdx),
    CallIndirect(TypeIdx),

    // Parametric Instructions:
    Drop,
    Select,

    // Variable Instructions:
    LocalGet(u32),
    LocalSet(u32),
    LocalTee(u32),
    GlobalGet(u32),
    GlobalSet(u32),

    // Memory Instructions:
    I32Load(MemArg),
    I64Load(MemArg),

    F32Load(MemArg),
    F64Load(MemArg),

    I32Load8S(MemArg),
    I32Load8U(MemArg),
    I32Load16S(MemArg),
    I32Load16U(MemArg),

    I64Load8S(MemArg),
    I64Load8U(MemArg),
    I64Load16S(MemArg),
    I64Load16U(MemArg),
    I64Load32S(MemArg),
    I64Load32U(MemArg),

    I32Store(MemArg),
    I64Store(MemArg),

    F32Store(MemArg),
    F64Store(MemArg),

    I32Store8(MemArg),
    I32Store16(MemArg),

    I64Store8(MemArg),
    I64Store16(MemArg),
    I64Store32(MemArg),
    MemSize,
    MemGrow,

    // Numeric Instructions:
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),

    // I32 Relop:
    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtI,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,

    // I64 Relop:
    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64LtI,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,

    // F32 Relop:
    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,

    // F64 Relop:
    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,

    // I32 Binop:
    I32Cls,
    I32Ctz,
    I32Popcnt,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32OrdXor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,

    // I64 Binop:
    I64Cls,
    I64Ctz,
    I64Popcnt,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64OrdXor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,

    // F32 Binop:
    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,

    // F64 Binop:
    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Copysign,

    // Cvtop:
    I32WrapI64,
    I32TruncF32S,
    I32TruncF32U,
    I32TruncF64S,
    I32TruncF64U,

    I64ExtendI32S,
    I64ExtendI32U,
    I64TruncF32S,
    I64TruncF32U,
    I64TruncF64S,
    I64TruncF64U,

    F32ConvertI32S,
    F32ConvertI32U,
    F32ConvertI64S,
    F32ConvertI64U,

    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64U,

    F64PromoteF32,

    I32ReinterpF32,
    I64ReinterpF64,
    F32ReinterpI32,
    F64ReinterpI64,
}

// impl From<&[u8]> for Instr {
//     fn from(data: &[u8]) -> Instr {
//         match data[0] {
//             0x00..=0x11 => {

//             }
//         }
//     }
// }

// fn parse_control_code(data: &[u8]) -> IResult<&[u8], Instr> {

// }
