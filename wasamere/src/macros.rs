// use crate::parser::Parse;
// use crate::leb_u32;
// use nom::IResult;

#[macro_export]
macro_rules! impl_index {
    ($id:ident) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        pub struct $id(pub(crate) u32);

        impl $id {
            pub fn index(&self) -> u32 {
                self.0
            }
        }

        impl crate::parser::Parse for $id {
            fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
                let (input, value) = crate::leb_u32(input)?;

                Ok((input, Self(value)))
            }
        }

        impl From<u32> for $id {
            fn from(val: u32) -> Self {
                Self(val)
            }
        }

        impl Into<u32> for $id {
            fn into(self) -> u32 {
                self.0
            }
        }
    };
}

#[macro_export]
macro_rules! test_parse {
    ($name:ident, $test:ty => $inst:expr, $bytes:expr) => {
        #[test]
        fn $name() {
            let input: &[u8] = $bytes;

            let (input, value) = <$test>::parse(&input).unwrap();

            assert!(input.is_empty());

            assert_eq!($inst, value);
        }
    };
}
