pub mod control;
pub mod numeric;
pub mod parametric;
pub mod variable;

#[derive(Debug, Clone, PartialEq)]
pub enum Instr {
    Numeric(numeric::Instr),
    Parametric(parametric::Instr),
    Variable(variable::Instr),
    Control(control::Instr),
}
