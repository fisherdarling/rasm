use nom::{IResult, le_u32, le_u8};

use crate::section::{
    CodeSection, ElementSection, ExportSection, FuncSection, GlobalSection, ImportSection,
    MemSection, StartSection, TableSection, TypeSection,
};

use crate::section::{
    parse_codesec,
    parse_elemsec,
    parse_exportsec,
    parse_funcsec,
    parse_globalsec,
    parse_importsec,
    parse_memsec,
    parse_startsec,
    parse_tablesec,
    parse_typesec
};

pub static MAGIC_NUMBER: u32 = 0x00_61_73_6D;
pub static VERSION: u32 = 0x01_00_00_00;

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub magic: u32,
    pub version: u32,
    // custom: CustomSection
    // pub data: DataSection
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

impl Default for Module {
    fn default() -> Self {
        use crate::types::index::FuncIdx;

        Module {
            magic: MAGIC_NUMBER,
            version: VERSION,
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

pub fn parse_module(input: &[u8]) -> IResult<&[u8], Module> {
    let mut module = Module::default();

    let (input, magic) = le_u32(input)?;
    let (mut input, version) = le_u32(input)?;

    module.magic = magic;
    module.version = version;

    // opt!(complete!())
    loop {
        let check = do_parse!(input,
            sec_code: opt!(complete!(call!(le_u8))) >>
            switch!(value!(sec_code),
                Some(1) => map!(parse_typesec, |sec| module.types = sec) |
                Some(2) => map!(parse_importsec, |sec| module.imports = sec) |
                Some(3) => map!(parse_funcsec, |sec| module.funcs = sec) |
                Some(4) => map!(parse_tablesec, |sec| module.tables = sec) |
                Some(5) => map!(parse_memsec, |sec| module.mems = sec) |
                Some(6) => map!(parse_globalsec, |sec| module.globals = sec) |
                Some(7) => map!(parse_exportsec, |sec| module.exports = sec) |
                Some(8) => map!(parse_startsec, |sec| module.start = sec) |
                Some(9) => map!(parse_elemsec, |sec| module.elem = sec) |
                Some(10) => map!(parse_codesec, |sec| module.code = sec)
            ) >>
            (sec_code)
        );

        match check {
            Ok((new_input, Some(code))) => {
                println!("Parsed section code {}", code);
                input = new_input;
            },
            _ => break,
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