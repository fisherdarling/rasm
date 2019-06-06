use nom::le_u8;
use nom::take;
use nom::ErrorKind;
use nom::IResult;
use nom::{le_f32, le_f64, le_u64};

use crate::error::Error;
use crate::instr::*;
use crate::types::index::*;
use crate::types::*;

use crate::leb_u32;

named!(
    pub parse_limit<Limit>,
    map!(
        switch!(le_u8,
            0x00 => count!(leb_u32, 1) |
            0x01 => count!(leb_u32, 2)
        ),
        |s| if s.len() == 1 {
            Limit {
                min: s[0],
                max: None,
            }
        } else {
            Limit {
                min: s[0],
                max: Some(s[1]),
            }
        }
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;
    use crate::StructNom;

    #[test]
    fn parse_valtype_vec() {
        // length: 4
        // values: I32, I64, F32, F64
        let bytes = [0x04, 0x7F, 0x7E, 0x7D, 0x7C];

        let (rest, types) = <Vec<ValType>>::nom(&bytes).unwrap();

        assert!(rest.is_empty());

        assert_eq!(
            &types,
            &[ValType::I32, ValType::I64, ValType::F32, ValType::F64]
        );
    }

    #[test]
    fn parse_var_instr() {
        let bytes = [0x20, 0x00];

        let (rest, instr) = Instr::nom(&bytes).unwrap();

        assert!(rest.is_empty());

        println!("{:?}", instr);
    }

    #[test]
    fn parse_functype_many() {
        let bytes = [0x60, 0x02, 0x7F, 0x7F, 0x01, 0x7F];

        let (rest, functype) = FuncType::nom(&bytes).unwrap();

        assert!(rest.is_empty());

        println!("{:?}", functype);
    }
}
