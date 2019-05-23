
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ValType {
    I32,
    I64,
    F32,
    F64,
}

impl From<u8> for ValType {
    fn from(code: u8) -> ValType {
        match code {
            0x7F => ValType::I32,
            0x7E => ValType::I64,
            0x7D => ValType::F32,
            0x7C => ValType::F64,
            _ => panic!("A Valtype cannot be created from the given byte."),
        }
    }
}

pub enum ResType {
    ValType(ValType),
    Unit,
}

impl ResType {
    pub const fn i_32() -> ResType {
        ResType::ValType(ValType::I32)
    }

    pub const fn i_64() -> ResType {
        ResType::ValType(ValType::I64)
    }

    pub const fn f_32() -> ResType {
        ResType::ValType(ValType::F32)
    }

    pub const fn f_64() -> ResType {
        ResType::ValType(ValType::F64)
    }

    pub const fn unit() -> ResType {
        ResType::Unit
    }
}

impl From<u8> for ResType {
    fn from(code: u8) -> ResType {
        match code {
            0x7F => ResType::i_32(),
            0x7E => ResType::i_64(),
            0x7D => ResType::f_32(),
            0x7C => ResType::f_64(),
            0x40 => ResType::unit(),
            _ => panic!("A Valtype cannot be created from the given byte."),
        }
    }
}

pub struct FuncType {
    params: Vec<ValType>,
    result: ValType,
}

impl FuncType {
    pub fn new(params: Vec<ValType>, result: Vec<ValType>) -> Self {
        Self {
            params,
            result: result[0],
        }
    }
}