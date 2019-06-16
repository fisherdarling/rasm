#![feature(cell_update, custom_attribute, try_reserve)]

pub mod error;
pub mod function;
pub mod instr;
pub mod module;
pub mod runtime;
pub mod store_old;
pub mod types;
pub mod value;
pub mod math;
pub mod macros;
pub mod store;

pub use macros::*;
