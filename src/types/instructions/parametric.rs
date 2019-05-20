#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Instr(Op);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Op {
    Drop,
    Select,
}
