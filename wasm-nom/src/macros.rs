// use crate::parser::Parse;
// use crate::leb_u32;
// use nom::IResult;

#[macro_export]
macro_rules! impl_leb32_wrapper {
    ($id:ident) => {
        #[derive(Copy, Clone, PartialEq, Eq, Hash)]
        pub struct $id(pub(crate) u32);

        impl $id {
            #[inline]
            pub fn index(&self) -> u32 {
                self.0
            }

            #[inline]
            pub fn as_usize(&self) -> usize {
                self.0 as usize
            }
        }

        impl crate::LEB32 for $id {}

        impl std::ops::Deref for $id {
            type Target = u32;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $id {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl From<u32> for $id {
            fn from(val: u32) -> Self {
                Self(val)
            }
        }

        impl From<usize> for $id {
            fn from(val: usize) -> Self {
                Self(val as u32)
            }
        }

        impl Into<u32> for $id {
            fn into(self) -> u32 {
                self.0
            }
        }

        impl std::fmt::Debug for $id {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "idx: {}", self.0)
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

            let (input, value) = <$test>::nom(&input).unwrap();

            assert!(input.is_empty());

            assert_eq!($inst, value);
        }
    };
    ($name:ident, $test:ty => $inst:expr, $bytes:expr, $debug:expr) => {
        #[test]
        fn $name() {
            let input: &[u8] = $bytes;

            let (input, value) = <$test>::nom(&input).unwrap();

            if $debug {
                println!("{:?}, {:?}", value, input);
            }

            assert!(input.is_empty());

            assert_eq!($inst, value);
        }
    };
}
