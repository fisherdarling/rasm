use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Encountered EOF while parsing")]
    EOF
}