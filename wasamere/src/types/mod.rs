mod types;

pub use types::*;

#[cfg(test)]
mod tests {
    use self::types::index::*;
    use self::types::*;
    use crate::parser::Parse;
    use crate::test_parse;

    test_parse!(parse_valtypes, Vec<ValType> => vec![ValType::I32, ValType::I64, ValType::F32, ValType::F64], &[0x04, 0x7F, 0x7E, 0x7D, 0x7C]);
}
