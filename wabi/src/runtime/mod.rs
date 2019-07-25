pub mod frame;
pub mod interpreter;
pub mod module_instance;
pub mod resolver;
pub mod runtime_store;
pub mod runtime;

pub use module_instance::*;

// use std::ops::{Index, IndexMut};

// pub struct Function;
// pub struct ModInst;

// pub struct ModIdx;
// pub struct FuncIdx;

// pub struct Res;

// pub trait RuntimeStore:
//     IndexMut<ModIdx, Output = ModInst> + Index<FuncIdx, Output = Function>
// {
// }

// pub trait Resolver {
//     fn resolve_function(&self, path: Option<impl AsRef<str>>, name: impl AsRef<str>) -> FuncIdx
//     where
//         Self: Sized;

//     fn resolve_module(&self, path: Option<impl AsRef<str>>) -> ModIdx
//     where
//         Self: Sized;
// }

// pub struct Runtime {
//     global_store: Box<dyn GlobalStore>,
//     resolver: Box<dyn Resolver>,
// }

// pub struct Interpreter<S, R>
//     where
//         S: GlobalStore,
//         R: Resolver,
// {}

// impl Interpreter<S, R> {
//     pub fn invoke_function(&mut self, func_idx: FuncIdx) -> Res;

//     pub fn invoke_name(&mut self, path: Option<impl AsRef<str>>, name: impl AsRef<str>) -> Res;

// }
