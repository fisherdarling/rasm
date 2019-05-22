pub mod binop;
pub mod cvtop;
pub mod relop;
pub mod unop;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Bitlen {
    N32,
    N64,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Signed {
    Signed,
    Unsigned,
}

pub mod testop {
    use super::Bitlen;

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Testop(Bitlen, Op);

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Op {
        Eqz,
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instr {
    Unop(unop::Unop),
    Binop(binop::Binop),
    Testop(testop::Testop),
    Relop(relop::Relop),
    Cvtop(cvtop::Cvtop),
}
