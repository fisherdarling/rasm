#![feature(cell_update)]

pub mod error;
pub mod function;
pub mod instr;
pub mod module;
pub mod runtime;
pub mod store;
pub mod types;
pub mod value;

pub mod macros;

pub use macros::*;
