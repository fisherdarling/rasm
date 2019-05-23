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

#[derive(Debug, Copy, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Limit {
    pub min: u32,
    pub max: Option<u32>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ElemType {
    FuncRef,
}

impl From<u8> for ElemType {
    fn from(code: u8) -> ElemType {
        match code {
            0x70 => ElemType::FuncRef,
            _ => panic!("Invalid code for elemtype"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TableType(pub ElemType, pub Limit);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mut {
    Const,
    Var
}

impl From<u8> for Mut {
    fn from(code: u8) -> Mut {
        match code {
            0x00 => Mut::Const,
            0x01 => Mut::Var,
            _ => panic!("Invalid code for mut"),
        }
    }
}

pub mod index {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct TypeIdx(u32);
    
    impl From<&[u8]> for TypeIdx {
        fn from(data: &[u8]) -> Self {
            use nom::le_u32;
            
            Self(le_u32(data).unwrap().1)
        }
    }

    impl TypeIdx {
        pub fn index(&self) -> u32 {
            self.0
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct FuncIdx(u32);

    impl From<&[u8]> for FuncIdx {
        fn from(data: &[u8]) -> Self {
            use nom::le_u32;
            
            Self(le_u32(data).unwrap().1)
        }
    }

    impl FuncIdx {
        pub fn index(&self) -> u32 {
            self.0
        }
    }
    
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct TableIdx(u32);

    impl From<&[u8]> for TableIdx {
        fn from(data: &[u8]) -> Self {
            use nom::le_u32;
            
            Self(le_u32(data).unwrap().1)
        }
    }

    impl TableIdx {
        pub fn index(&self) -> u32 {
            self.0
        }
    }
    
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct MemIdx(u32);

    impl From<&[u8]> for MemIdx {
        fn from(data: &[u8]) -> Self {
            use nom::le_u32;
            
            Self(le_u32(data).unwrap().1)
        }
    }

    impl MemIdx {
        pub fn index(&self) -> u32 {
            self.0
        }
    }
    
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct GlobalIdx(u32);

    impl From<&[u8]> for GlobalIdx {
        fn from(data: &[u8]) -> Self {
            use nom::le_u32;
            
            Self(le_u32(data).unwrap().1)
        }
    }

    impl GlobalIdx {
        pub fn index(&self) -> u32 {
            self.0
        }
    }
    
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct LocalIdx(u32);

    impl From<&[u8]> for LocalIdx {
        fn from(data: &[u8]) -> Self {
            use nom::le_u32;
            
            Self(le_u32(data).unwrap().1)
        }
    }

    impl LocalIdx {
        pub fn index(&self) -> u32 {
            self.0
        }
    }
    
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct LabelIdx(u32);

    impl From<&[u8]> for LabelIdx {
        fn from(data: &[u8]) -> Self {
            use nom::le_u32;
            
            Self(le_u32(data).unwrap().1)
        }
    }

    impl LabelIdx {
        pub fn index(&self) -> u32 {
            self.0
        }
    }


}