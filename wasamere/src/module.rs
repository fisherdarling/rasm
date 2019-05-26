use std::fs::File;
use std::io::Read;
use std::path::Path;

use nom::{le_u32, le_u8, IResult};

use crate::parser::Parse;

use crate::leb_u32;

use crate::section::{
    CodeSection, CustomSection, DataSection, ElementSection, ExportSection, FuncSection,
    GlobalSection, ImportSection, MemSection, StartSection, TableSection, TypeSection,
};

pub static MAGIC_NUMBER: u32 = 0x00_61_73_6D;
pub static VERSION: u32 = 0x01_00_00_00;

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub magic: u32,
    pub version: u32,
    pub custom: CustomSection,
    pub data: DataSection,
    pub types: TypeSection,
    pub funcs: FuncSection,
    pub code: CodeSection,
    pub tables: TableSection,
    pub mems: MemSection,
    pub globals: GlobalSection,
    pub elem: ElementSection,
    pub start: StartSection,
    pub imports: ImportSection,
    pub exports: ExportSection,
}

impl Module {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Module {
        let mut file = File::open(path).unwrap();

        let mut bytes: Vec<u8> = Vec::new();
        println!("Read {} bytes", file.read_to_end(&mut bytes).unwrap());

        let (_, module) = parse_module(&bytes).unwrap();

        module
    }

    pub fn from_bytes(input: &[u8]) -> Module {
        let (_, module) = parse_module(&input).unwrap();

        module
    }
}

impl Default for Module {
    fn default() -> Self {
        use crate::types::index::FuncIdx;

        Module {
            magic: MAGIC_NUMBER,
            version: VERSION,
            custom: CustomSection(String::new(), Vec::new()),
            data: DataSection(Vec::new()),
            types: TypeSection(Vec::new()),
            funcs: FuncSection(Vec::new()),
            code: CodeSection(Vec::new()),
            tables: TableSection(Vec::new()),
            mems: MemSection(Vec::new()),
            globals: GlobalSection(Vec::new()),
            elem: ElementSection(Vec::new()),
            start: StartSection(FuncIdx(std::u32::MAX)),
            imports: ImportSection(Vec::new()),
            exports: ExportSection(Vec::new()),
        }
    }
}

named!(
    verify_header,
    preceded!(
        tag!(MAGIC_NUMBER.to_be_bytes()),
        tag!(VERSION.to_be_bytes())
    )
);

pub fn parse_module(input: &[u8]) -> IResult<&[u8], Module> {
    let mut module = Module::default();

    let (input, magic) = le_u32(input)?;
    let (mut input, version) = le_u32(input)?;

    module.magic = magic;
    module.version = version;

    loop {
        let (rest, code) = do_parse!(
            input,
            sec_code: tap!( res: opt!(complete!(call!(le_u8))) => { println!("[code] {:?}", res) })
                >> sec_size:
                    tap!( res: opt!(complete!(call!(leb_u32))) => {println!("[size] {:?}", res) })
                >> opt!(switch!(value!(sec_code),
                    Some(0) => map!(Parse::parse, |sec| { println!("{:?}", sec); module.custom = sec }) |
                    Some(1) => map!(Parse::parse, |sec| { println!("{:?}", sec); module.types = sec }) |
                    Some(2) => map!(Parse::parse, |sec| { println!("{:?}", sec); module.imports = sec }) |
                    Some(3) => map!(Parse::parse, |sec| { println!("{:?}", sec); module.funcs = sec }) |
                    Some(4) => map!(Parse::parse, |sec| { println!("{:?}", sec); module.tables = sec }) |
                    Some(5) => map!(Parse::parse, |sec| { println!("{:?}", sec); module.mems = sec }) |
                    Some(6) => map!(Parse::parse, |sec| { println!("{:?}", sec); module.globals = sec }) |
                    Some(7) => map!(Parse::parse, |sec| { println!("{:?}", sec); module.exports = sec }) |
                    Some(8) => map!(Parse::parse, |sec| { println!("{:?}", sec); module.start = sec }) |
                    Some(9) => map!(Parse::parse, |sec| { println!("{:?}", sec); module.elem = sec }) |
                    Some(10) => map!(Parse::parse, |sec| { println!("{:?}", sec); module.code = sec }) |
                    Some(11) => map!(Parse::parse, |sec| { println!("{:?}", sec); module.data = sec }) |
                    Some(c) => map!(take!(0), |_| println!("Got: {}", c)) |
                    _ => value!(())
                ))
                >> (sec_code)
        )?;

        println!("Input length: {}", input.len());

        input = rest;

        if code.is_none() {
            break;
        }
    }

    Ok((input, module))
}

// named!(parse_module<Module>,
//     do_parse!(
//         magic: call!(le_u32) >>
//         version: call!(le_u32) >>
//         (Module::default())
//     )

// );
