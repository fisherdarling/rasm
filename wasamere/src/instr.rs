use crate::types::index::{Align, FuncIdx, GlobalIdx, LabelIdx, LocalIdx, Offset, TypeIdx};
use crate::types::ResType;
use std::ops::{Deref, DerefMut};

use crate::{leb_u32, StructNom};
use nom::{le_u8, IResult};

#[derive(Debug, Clone, PartialEq, StructNom)]
pub struct Expression(pub Vec<Instr>);

impl StructNom for Vec<Instr> {
    fn nom(input: &[u8]) -> IResult<&[u8], Self> {
        let (rest, mut instrs) = do_parse!(
            input,
            instrs: many_till!(Instr::nom, tag!(&[0x0B])) >> (instrs.0)
        )?;

        instrs.push(Instr::End);

        Ok((rest, instrs))
    }
}

impl Deref for Expression {
    type Target = [Instr];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Expression {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq, StructNom)]
#[snom(switch = le_u8)]
pub enum Instr {
    // Control Instructions:
    #[snom(range(start = 0x00))]
    Unreachable,
    Nop,
    Block(ResType, Expression),
    Loop(ResType, Expression),
    #[snom(parser = parse_if)]
    If(ResType, Expression, Expression),
    #[snom(range(end = 0x05))]
    End,
    #[snom(range(start = 0x0B))]
    Else,
    Br(LabelIdx),
    BrIf(LabelIdx),
    // Figure out meaning of l_N
    BrTable(Vec<LabelIdx>, LabelIdx),
    Return,
    Call(FuncIdx),
    #[snom(range(end = 0x11))]
    CallIndirect(TypeIdx),

    // Parametric Instructions:
    #[snom(val = 0x1A)]
    Drop,
    #[snom(val = 0x1B)]
    Select,

    // Variable Instructions:
    #[snom(range(start = 0x20))]
    LocalGet(LocalIdx),
    LocalSet(LocalIdx),
    LocalTee(LocalIdx),
    GlobalGet(GlobalIdx),
    #[snom(range(end = 0x24))]
    GlobalSet(GlobalIdx),

    // Memory Instructions:
    #[snom(range(start = 0x28))]
    I32Load(Align, Offset),
    I64Load(Align, Offset),

    F32Load(Align, Offset),
    F64Load(Align, Offset),

    I32Load8S(Align, Offset),
    I32Load8U(Align, Offset),
    I32Load16S(Align, Offset),
    I32Load16U(Align, Offset),

    I64Load8S(Align, Offset),
    I64Load8U(Align, Offset),
    I64Load16S(Align, Offset),
    I64Load16U(Align, Offset),
    I64Load32S(Align, Offset),
    I64Load32U(Align, Offset),

    I32Store(Align, Offset),
    I64Store(Align, Offset),

    F32Store(Align, Offset),
    F64Store(Align, Offset),

    I32Store8(Align, Offset),
    I32Store16(Align, Offset),

    I64Store8(Align, Offset),
    I64Store16(Align, Offset),
    I64Store32(Align, Offset),
    MemSize,
    #[snom(range(end = 0x40))]
    MemGrow,

    // Numeric Instructions:
    #[snom(range(start = 0x41))]
    I32Const(#[snom(parser = leb_u32)] u32),
    I64Const(u64),
    F32Const(f32),
    F64Const(f64),

    // I32 Relop:
    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
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
    I64LtU,
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
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,

    // I64 Binop:
    I64Clz,
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
    I64Or,
    I64Xor,
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
    F32DemoteF64,

    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64U,

    F64PromoteF32,

    I32ReinterpF32,
    I64ReinterpF64,
    F32ReinterpI32,
    #[snom(range(end = 0xBF))]
    F64ReinterpI64,
}

named!(
    parse_if<Instr>,
    do_parse!(
        // value!(println!("Parse If")) >>
        restype: call!(ResType::nom)
        // value!(println!("ResType: {:?}", restype))
            >> conseq: many_till!(Instr::nom, one_of!(&[0x0B, 0x05]))
            // value!(println!("Conseq: {:?}", conseq))
            >> altern:
                switch!(value!(conseq.1 as u8),
                    0x05 => call!(Expression::nom) |
                    0x0B => value!(Expression(vec![Instr::End])))
            >> conseq: map!(value!(conseq.0), |mut c| { c.push(Instr::End); Expression(c) })
            >> (Instr::If(restype, conseq, altern))
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_parse;

    use crate::types::*;

    static IF_STMT: &[u8] = &[
        0x04, // If
        0x7f, // ResType: i32
        0x20, 0x00, // local.get: 0
        0x20, 0x01, // local.get: 1
        0x10, 0x0f, // call: FuncIdx(15)
        0x2d, 0x00, 0x00, // i32.load8_u { align: 0, offset: 0 }
        // Else
        0x05, 0x41, 0x00, // i32.const: 0
        0x0b, // end (else)
    ];

    static BLOCK: &[u8] = &[
        0x02, 0x7f, // Instr (2): Block | ResType: I32
        0x41, 0x00, // i32.const: 0
        0x41, 0x32, // i32.const: 0x32
        0x20, 0x00, // local.get: 0
        0x10, 0x05, // call: FuncIdx(5)
        0x41, 0x00, // i32.const: 0
        0x41, 0x32, // i32.const: 0x32
        0x20, 0x01, // local.get: 1
        0x10, 0x0a, // call: FuncIdx(10)
        0x71, // i32.and
        0x0b, // end
    ];

    test_parse!(
        parse_block,
        Instr => Instr::Block(
            ResType::I32,
            Expression(vec![
                Instr::I32Const(0),
                Instr::I32Const(0x32),
                Instr::LocalGet(0.into()),
                Instr::Call(5.into()),
                Instr::I32Const(0),
                Instr::I32Const(0x32),
                Instr::LocalGet(1.into()),
                Instr::Call(10.into()),
                Instr::I32And,
                Instr::End
            ])
        ),
        BLOCK
    );

    test_parse!(
        parse_expression,
        Expression => Expression(vec![
            Instr::LocalGet(LocalIdx(0)),
            Instr::LocalGet(LocalIdx(1)),
            Instr::I32Add,
            Instr::End,
        ]),
        &[0x20, 0x00, 0x20, 0x01, 0x6a, 0x0b]
    );

    test_parse!(
        parse_if,
        Instr => Instr::If(ResType::I32, Expression(vec![
            Instr::LocalGet(LocalIdx(0)),
            Instr::LocalGet(LocalIdx(1)),
            Instr::Call(FuncIdx(15)),
            Instr::I32Load8U(0.into(), 0.into()),
            Instr::End,
        ]), Expression(vec![
            Instr::I32Const(0),
            Instr::End,
        ])),
        IF_STMT,
        true
    );
}
