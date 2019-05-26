use crate::parser::{PResult, Parse};
use crate::types::index::{FuncIdx, GlobalIdx, LabelIdx, LocalIdx, TypeIdx};
use crate::types::ResType;

use crate::leb_u32;
use nom::{le_f32, le_f64, le_u32, le_u64, le_u8, IResult};

pub type MemArg = (u32, u32);

#[derive(Debug, Clone, PartialEq, Parse)]
pub struct Expression(pub Vec<Instr>);

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
    LocalGet(LocalIdx),
    LocalSet(LocalIdx),
    LocalTee(LocalIdx),
    GlobalGet(GlobalIdx),
    GlobalSet(GlobalIdx),

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
    I32Const(u32),
    I64Const(u64),
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
    F64ReinterpI64,
}

impl Parse for Instr {
    fn parse(input: &[u8]) -> PResult<Self> {
        parse_instr(input)
    }
}

named!(
    parse_block<Instr>,
    do_parse!(
        restype: call!(ResType::parse)
            >> instrs: many_till!(parse_instr, tag!(&[0x0B]))
            >> (Instr::Block(ResType::from(restype), instrs.0))
    )
);

named!(
    parse_loop<Instr>,
    do_parse!(
        restype: call!(ResType::parse)
            >> instrs: many_till!(parse_instr, tag!(&[0x0B]))
            >> (Instr::Loop(restype, instrs.0))
    )
);

named!(
    parse_if<Instr>,
    do_parse!(
        restype: call!(ResType::parse)
            >> conseq: many_till!(parse_instr, alt!(tag!(&[0x0B]) | tag!(&[0x0F])))
            >> altern:
                switch!(value!(conseq.1),
                    &[0x0F] => map!(many_till!(parse_instr, tag!(&[0x0B])), |s| s.0) |
                    &[0x0B] => value!(Vec::<Instr>::new())
                )
            >> (Instr::If(restype, conseq.0, altern))
    )
);

pub fn parse_control_instr(input: &[u8], code: u8) -> PResult<Instr> {
    switch!(input, value!(code),
        // Control Instructions:
        0x00 => value!(Instr::Unreachable) |
        0x01 => value!(Instr::Nop) |
        // Block
        0x02 => call!(parse_block) |
        // Loop
        0x03 => call!(parse_loop) |
        // If
        0x04 => call!(parse_if) |
        // Br label
        0x0C => do_parse!(
            index: call!(LabelIdx::parse) >>
            (Instr::Br(index))
        ) |
        // Br If
        0x0D => do_parse!(
            index: call!(LabelIdx::parse) >>
            (Instr::BrIf(index))
        ) |
        // Br Table
        0x0E => do_parse!(
            indices: call!(Parse::parse) >>
            index: call!(LabelIdx::parse) >>
            (Instr::BrTable(indices, index))
        ) |
        0x0F => value!(Instr::Return) |
        0x10 => do_parse!(
            index: call!(FuncIdx::parse) >>
            (Instr::Call(index))
        ) |
        0x11 => do_parse!(
            index: call!(TypeIdx::parse) >>
            tag!(&[0x00]) >>
            (Instr::CallIndirect(index))
        )
    )
}

// pub fn parse_control_instr(input: &[u8], code: u8) -> IResult<&[u8], Instr> {
// }

pub fn parse_parametric_instr(input: &[u8], code: u8) -> IResult<&[u8], Instr> {
    switch!(input, value!(code),
        0x1A => value!(Instr::Drop) |
        0x1B => value!(Instr::Select)
    )
}

pub fn parse_variable_instr(input: &[u8], code: u8) -> IResult<&[u8], Instr> {
    switch!(input, value!(code),
        0x20 => do_parse!(
            index: call!(LocalIdx::parse) >>
            (Instr::LocalGet(index))
        ) |
        0x21 => do_parse!(
            index: call!(LocalIdx::parse) >>
            (Instr::LocalSet(index))
        ) |
        0x22 => do_parse!(
            index: call!(LocalIdx::parse) >>
            (Instr::LocalTee(index))
        ) |
        0x23 => do_parse!(
            index: call!(GlobalIdx::parse) >>
            (Instr::GlobalGet(index))
        ) |
        0x24 => do_parse!(
            index: call!(GlobalIdx::parse) >>
            (Instr::GlobalSet(index))
        )
    )
}

pub fn parse_mem_instr(input: &[u8], code: u8) -> IResult<&[u8], Instr> {
    switch!(input, value!(code),
            // Memory Instructions:
        0x28 => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I32Load((align, offset)))) |
        0x29 => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I64Load((align, offset)))) |
        0x2A => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::F32Load((align, offset)))) |
        0x2B => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::F64Load((align, offset)))) |
        0x2C => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I32Load8S((align, offset)))) |
        0x2D => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I32Load8U((align, offset)))) |
        0x2E => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I32Load16S((align, offset)))) |
        0x2F => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I32Load16U((align, offset)))) |
        0x30 => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I64Load8S((align, offset)))) |
        0x31 => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I64Load8U((align, offset)))) |
        0x32 => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I64Load16S((align, offset)))) |
        0x33 => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I64Load16U((align, offset)))) |
        0x34 => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I64Load32S((align, offset)))) |
        0x35 => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I64Load32U((align, offset)))) |
        0x36 => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I32Store((align, offset)))) |
        0x37 => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I64Store((align, offset)))) |
        0x38 => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::F32Store((align, offset)))) |
        0x39 => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::F64Store((align, offset)))) |
        0x3A => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I32Store8((align, offset)))) |
        0x3B => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I32Store16((align, offset)))) |
        0x3C => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I64Store8((align, offset)))) |
        0x3D => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I64Store16((align, offset)))) |
        0x3E => do_parse!(
            align: call!(leb_u32) >>
            offset: call!(leb_u32) >>
            (Instr::I64Store32((align, offset)))) |
        0x3F => do_parse!(
            tag!(&[0x00]) >>
            (Instr::MemSize)) |
        0x40 => do_parse!(
            tag!(&[0x00]) >>
            (Instr::MemGrow))
    )
}

pub fn parse_num_instr(input: &[u8], code: u8) -> IResult<&[u8], Instr> {
    switch!(input, value!(code),
        0x41 => do_parse!(
           imm: call!(leb_u32) >>
           (Instr::I32Const(imm))  
        ) |
        0x42 => do_parse!(
           imm: call!(le_u64) >>
           (Instr::I64Const(imm))  
        ) |
        0x43 => do_parse!(
           imm: call!(le_f32) >>
           (Instr::F32Const(imm))  
        ) |
        0x44 => do_parse!(
           imm: call!(le_f64) >>
           (Instr::F64Const(imm))  
        ) |
        0x45 =>	value!(Instr::I32Eqz) |
        0x46 =>	value!(Instr::I32Eq) |
        0x47 =>	value!(Instr::I32Ne) |
        0x48 =>	value!(Instr::I32LtS) |
        0x49 =>	value!(Instr::I32LtI) |
        0x4a =>	value!(Instr::I32GtS) |
        0x4b =>	value!(Instr::I32GtU) |
        0x4c =>	value!(Instr::I32LeS) |
        0x4d =>	value!(Instr::I32LeU) |
        0x4e =>	value!(Instr::I32GeS) |
        0x4f =>	value!(Instr::I32GeU) |
        0x50 =>	value!(Instr::I64Eqz) |
        0x51 =>	value!(Instr::I64Eq) |
        0x52 =>	value!(Instr::I64Ne) |
        0x53 =>	value!(Instr::I64LtS) |
        0x54 =>	value!(Instr::I64LtI) |
        0x55 =>	value!(Instr::I64GtS) |
        0x56 =>	value!(Instr::I64GtU) |
        0x57 =>	value!(Instr::I64LeS) |
        0x58 =>	value!(Instr::I64LeU) |
        0x59 =>	value!(Instr::I64GeS) |
        0x5a =>	value!(Instr::I64GeU) |
        0x5b =>	value!(Instr::F32Eq) |
        0x5c =>	value!(Instr::F32Ne) |
        0x5d =>	value!(Instr::F32Lt) |
        0x5e =>	value!(Instr::F32Gt) |
        0x5f =>	value!(Instr::F32Le) |
        0x60 =>	value!(Instr::F32Ge) |
        0x61 =>	value!(Instr::F64Eq) |
        0x62 =>	value!(Instr::F64Ne) |
        0x63 =>	value!(Instr::F64Lt) |
        0x64 =>	value!(Instr::F64Gt) |
        0x65 =>	value!(Instr::F64Le) |
        0x66 =>	value!(Instr::F64Ge) |
        0x67 =>	value!(Instr::I32Cls) |
        0x68 =>	value!(Instr::I32Ctz) |
        0x69 =>	value!(Instr::I32Popcnt) |
        0x6a =>	value!(Instr::I32Add) |
        0x6b =>	value!(Instr::I32Sub) |
        0x6c =>	value!(Instr::I32Mul) |
        0x6d =>	value!(Instr::I32DivS) |
        0x6e =>	value!(Instr::I32DivU) |
        0x6f =>	value!(Instr::I32RemS) |
        0x70 =>	value!(Instr::I32RemU) |
        0x71 =>	value!(Instr::I32And) |
        0x72 =>	value!(Instr::I32Or) |
        0x73 =>	value!(Instr::I32Xor) |
        0x74 =>	value!(Instr::I32Shl) |
        0x75 =>	value!(Instr::I32ShrS) |
        0x76 =>	value!(Instr::I32ShrU) |
        0x77 =>	value!(Instr::I32Rotl) |
        0x78 =>	value!(Instr::I32Rotr) |
        0x79 =>	value!(Instr::I64Clz) |
        0x7a =>	value!(Instr::I64Ctz) |
        0x7b =>	value!(Instr::I64Popcnt) |
        0x7c =>	value!(Instr::I64Add) |
        0x7d =>	value!(Instr::I64Sub) |
        0x7e =>	value!(Instr::I64Mul) |
        0x7f =>	value!(Instr::I64DivS) |
        0x80 =>	value!(Instr::I64DivU) |
        0x81 =>	value!(Instr::I64RemS) |
        0x82 =>	value!(Instr::I64RemU) |
        0x83 =>	value!(Instr::I64And) |
        0x84 =>	value!(Instr::I64Or) |
        0x85 =>	value!(Instr::I64Xor) |
        0x86 =>	value!(Instr::I64Shl) |
        0x87 =>	value!(Instr::I64ShrS) |
        0x88 =>	value!(Instr::I64ShrU) |
        0x89 =>	value!(Instr::I64Rotl) |
        0x8a =>	value!(Instr::I64Rotr) |
        0x8b =>	value!(Instr::F32Abs) |
        0x8c =>	value!(Instr::F32Neg) |
        0x8d =>	value!(Instr::F32Ceil) |
        0x8e =>	value!(Instr::F32Floor) |
        0x8f =>	value!(Instr::F32Trunc) |
        0x90 =>	value!(Instr::F32Nearest) |
        0x91 =>	value!(Instr::F32Sqrt) |
        0x92 =>	value!(Instr::F32Add) |
        0x93 =>	value!(Instr::F32Sub) |
        0x94 =>	value!(Instr::F32Mul) |
        0x95 =>	value!(Instr::F32Div) |
        0x96 =>	value!(Instr::F32Min) |
        0x97 =>	value!(Instr::F32Max) |
        0x98 =>	value!(Instr::F32Copysign) |
        0x99 =>	value!(Instr::F64Abs) |
        0x9a =>	value!(Instr::F64Neg) |
        0x9b =>	value!(Instr::F64Ceil) |
        0x9c =>	value!(Instr::F64Floor) |
        0x9d =>	value!(Instr::F64Trunc) |
        0x9e =>	value!(Instr::F64Nearest) |
        0x9f =>	value!(Instr::F64Sqrt) |
        0xa0 =>	value!(Instr::F64Add) |
        0xa1 =>	value!(Instr::F64Sub) |
        0xa2 =>	value!(Instr::F64Mul) |
        0xa3 =>	value!(Instr::F64Div) |
        0xa4 =>	value!(Instr::F64Min) |
        0xa5 =>	value!(Instr::F64Max) |
        0xa6 =>	value!(Instr::F64Copysign) |
        0xa7 =>	value!(Instr::I32WrapI64) |
        0xa8 =>	value!(Instr::I32TruncF32S) |
        0xa9 =>	value!(Instr::I32TruncF32U) |
        0xaa =>	value!(Instr::I32TruncF64S) |
        0xab =>	value!(Instr::I32TruncF64U) |
        0xac =>	value!(Instr::I64ExtendI32S) |
        0xad =>	value!(Instr::I64ExtendI32U) |
        0xae =>	value!(Instr::I64TruncF32S) |
        0xaf =>	value!(Instr::I64TruncF32U) |
        0xb0 =>	value!(Instr::I64TruncF64S) |
        0xb1 =>	value!(Instr::I64TruncF64U) |
        0xb2 =>	value!(Instr::F32ConvertI32S) |
        0xb3 =>	value!(Instr::F32ConvertI32U) |
        0xb4 =>	value!(Instr::F32ConvertI64S) |
        0xb5 =>	value!(Instr::F32DemoteF64) |
        0xb6 =>	value!(Instr::F32ConvertI64U) |
        0xb7 =>	value!(Instr::F64ConvertI32S) |
        0xb8 =>	value!(Instr::F64ConvertI32U) |
        0xb9 =>	value!(Instr::F64ConvertI64S) |
        0xba =>	value!(Instr::F64ConvertI64U) |
        0xbb =>	value!(Instr::F64PromoteF32) |
        0xbc =>	value!(Instr::I32ReinterpF32) |
        0xbd =>	value!(Instr::I64ReinterpF64) |
        0xbe =>	value!(Instr::F32ReinterpI32) |
        0xbf =>	value!(Instr::F64ReinterpI64))
}

named!(
    parse_instr<Instr>,
    do_parse!(
        code: call!(le_u8) >>
        // instr: switch!(value!(code),
        instr:
            switch!(value!(code),
                // Control Instructions
                0x00..=0x11 => call!(parse_control_instr, code) |
                
                // Parametric Instructions:
                0x1A..=0x1B => call!(parse_parametric_instr, code) |
                
                // Variable Instructions:
                0x20..=0x24 => call!(parse_variable_instr, code) |
                
                // Mem Instructions:
                0x28..=0x40 => call!(parse_mem_instr, code) |

                // Numeric Instructions:
                0x41..=0xB4 => call!(parse_num_instr, code)
            )
            >> (instr)
    )
);
