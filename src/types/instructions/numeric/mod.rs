pub mod unop;
pub mod binop;
pub mod relop;
pub mod cvtop;

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

    pub struct Testop(Bitlen, Op);

    pub enum Op {
        Eqz,
    }
}

pub enum Instr {
    Unop(unop::Unop),
    Binop(binop::Binop),
    Testop(testop::Testop),
    Relop(relop::Relop),
    Cvtop(cvtop::Cvtop),
}