pub mod binop;
// pub mod numeric;

// pub use 

pub enum Bitlen {
    N32,
    N64,
} 

pub enum Signed {
    Signed,
    Unsigned,
}


pub mod testop {
    use super::Bitlen;

    pub struct itestop(Bitlen, Op);

    pub enum Op {
        Eqz,
    }
}