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

use crate::section::section::Section;
use crate::StructNom;

pub static MAGIC_NUMBER: u32 = 0x00_61_73_6D;
pub static VERSION: u32 = 0x01_00_00_00;

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedModule {
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
    pub start: Option<StartSection>,
    pub imports: ImportSection,
    pub exports: ExportSection,
}

impl ParsedModule {
    pub fn from_file<P: AsRef<Path>>(path: P) -> ParsedModule {
        let mut file = File::open(path).unwrap();

        let mut bytes: Vec<u8> = Vec::new();
        println!("Read {} bytes", file.read_to_end(&mut bytes).unwrap());

        let (_, module) = parse_module(&bytes).unwrap();

        module
    }

    pub fn from_bytes(input: &[u8]) -> ParsedModule {
        let (_, module) = parse_module(&input).unwrap();

        module
    }
}

impl Default for ParsedModule {
    fn default() -> Self {
        use crate::types::index::FuncIdx;

        ParsedModule {
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
            start: Some(StartSection(FuncIdx(std::u32::MAX))),
            imports: ImportSection(Vec::new()),
            exports: ExportSection(Vec::new()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseModuleBuilder {
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
    pub start: Option<StartSection>,
    pub imports: ImportSection,
    pub exports: ExportSection,
}

impl Default for ParseModuleBuilder {
    fn default() -> Self {
        use crate::types::index::FuncIdx;

        ParseModuleBuilder {
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
            start: Some(StartSection(FuncIdx(std::u32::MAX))),
            imports: ImportSection(Vec::new()),
            exports: ExportSection(Vec::new()),
        }
    }
}

impl ParseModuleBuilder {
    pub fn magic(self, magic: u32) -> Self {
        Self { magic, ..self }
    }

    pub fn version(self, version: u32) -> Self {
        Self { version, ..self }
    }

    pub fn custom(self, custom: CustomSection) -> Self {
        Self { custom, ..self }
    }

    pub fn data(self, data: DataSection) -> Self {
        Self { data, ..self }
    }
    pub fn types(self, types: TypeSection) -> Self {
        Self { types, ..self }
    }
    pub fn funcs(self, funcs: FuncSection) -> Self {
        Self { funcs, ..self }
    }
    pub fn code(self, code: CodeSection) -> Self {
        Self { code, ..self }
    }
    pub fn tables(self, tables: TableSection) -> Self {
        Self { tables, ..self }
    }

    pub fn mems(self, mems: MemSection) -> Self {
        Self { mems, ..self }
    }

    pub fn globals(self, globals: GlobalSection) -> Self {
        Self { globals, ..self }
    }

    pub fn elem(self, elem: ElementSection) -> Self {
        Self { elem, ..self }
    }

    pub fn start(self, start: StartSection) -> Self {
        Self {
            start: Some(start),
            ..self
        }
    }

    pub fn imports(self, imports: ImportSection) -> Self {
        Self { imports, ..self }
    }

    pub fn exports(self, exports: ExportSection) -> Self {
        Self { exports, ..self }
    }

    pub fn build(self) -> ParsedModule {
        ParsedModule {
            magic: self.magic,
            version: self.version,
            custom: self.custom,
            data: self.data,
            types: self.types,
            funcs: self.funcs,
            code: self.code,
            tables: self.tables,
            mems: self.mems,
            globals: self.globals,
            elem: self.elem,
            start: self.start,
            imports: self.imports,
            exports: self.exports,
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

pub fn parse_module(input: &[u8]) -> IResult<&[u8], ParsedModule> {
    let mut module = ParsedModule::default();

    let (input, magic) = le_u32(input)?;
    let (mut input, version) = le_u32(input)?;

    module.magic = magic;
    module.version = version;

    loop {
        if input.is_empty() {
            break;
        }

        // let (rest, sec_code) = le_u8(input)?;
        // let (rest, sec_size) = leb_u32(rest)?;

        // let rest =

        // println!("Input length: {}", input.len());
        let (rest, code) = do_parse!(
            input,
            sec_code: opt!(complete!(call!(le_u8)))
                >> sec_size: opt!(complete!(call!(leb_u32)))
                >> switch!(value!(sec_code),
                    Some(0) => map!(Parse::parse, |sec| { /* println!("{:#?}", sec); */ module.custom = sec }) |
                    Some(1) => map!(Parse::parse, |sec| { /* println!("{:#?}", sec); */ module.types = sec }) |
                    Some(2) => map!(Parse::parse, |sec| { /* println!("{:#?}", sec); */ module.imports = sec }) |
                    Some(3) => map!(Parse::parse, |sec| { /* println!("{:#?}", sec); */ module.funcs = sec }) |
                    Some(4) => map!(Parse::parse, |sec| { /* println!("{:#?}", sec); */ module.tables = sec }) |
                    Some(5) => map!(Parse::parse, |sec| { /* println!("{:#?}", sec); */ module.mems = sec }) |
                    Some(6) => map!(Parse::parse, |sec| { /* println!("{:#?}", sec); */ module.globals = sec }) |
                    Some(7) => map!(Parse::parse, |sec| { /* println!("{:#?}", sec); */ module.exports = sec }) |
                    Some(8) => map!(Parse::parse, |sec| { /* println!("{:#?}", sec); */ module.start = Some(sec) }) |
                    Some(9) => map!(Parse::parse, |sec| { /* println!("{:#?}", sec); */ module.elem = sec }) |
                    Some(10) => map!(Parse::parse, |sec| { /* println!("{:#?}", sec); */ module.code = sec }) |
                    Some(11) => map!(Parse::parse, |sec| { /* println!("{:#?}", sec); */ module.data = sec }) |
                    Some(c) => map!(take!(0), |_| println!("Got: {}", c)) |
                    _ => value!(())
                )
                >> (sec_code)
        )?;

        input = rest;

        if code.is_none() {
            break;
        }
    }

    Ok((input, module))
}

#[derive(Debug, Default, Clone, PartialEq, StructNom)]
pub struct TestModule {
    #[parser = "nom::le_u32"]
    magic: u32,
    #[parser = "nom::le_u32"]
    version: u32,
    sections: Vec<Section>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module() {
        let bytes = include_bytes!("../examples/add.wasm");

        let (rest, test_module) = TestModule::nom(bytes).unwrap();

        println!("{:?}", test_module);


    }
}

// named!(parse_module<Module>,
//     do_parse!(
//         magic: call!(le_u32) >>
//         version: call!(le_u32) >>
//         (Module::default())
//     )

// );
