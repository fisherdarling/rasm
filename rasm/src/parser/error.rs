use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Encountered EOF while parsing")]
    EOF,
    #[fail(display = "Invalid limit")]
    InvalidLimit,
    #[fail(display = "Invalid Value Type")]
    InvalidValueType,
    #[fail(display = "Invalid Elem Type")]
    InvalidElemType,
    #[fail(display = "Invalid Table Type")]
    InvalidTableType,
    #[fail(display = "Invalid Mutability Type")]
    InvalidMutability,
}
