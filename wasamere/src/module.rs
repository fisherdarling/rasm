use crate::section::Section;
use crate::StructNom;

pub static MAGIC_NUMBER: &[u8] = &[0x00, 0x61, 0x73, 0x6D];
pub static VERSION: u32 = 0x01_00_00_00;

#[derive(Debug, Default, Clone, PartialEq, StructNom)]
pub struct ParsedModule {
    #[snom(tag(MAGIC_NUMBER))] // All modules start with the magic number
    #[snom(take(4))]           // The version is always 4 bytes
    sections: Vec<Section>
}

impl ParsedModule {
    pub fn sections(&self) -> &[Section] {
        &self.sections
    }

    pub fn parse_bytes(input: &[u8]) -> Self {
        ParsedModule::nom(input).unwrap().1
    }
}

impl StructNom for Vec<Section> {
    fn nom(input: &[u8]) -> nom::IResult<&[u8], Self> {
        let mut sections = Vec::new();

        let (mut input, mut next) = opt!(input, complete!(Section::nom))?;

        while let Some(sec) = next {
            sections.push(sec);

            let (new_input, new_next) = opt!(input, complete!(Section::nom))?;
            input = new_input;
            next = new_next;
        }

        Ok((input, sections))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module() {
        let bytes = include_bytes!("../examples/add.wasm");

        let (_rest, test_module) = ParsedModule::nom(bytes).unwrap();

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
