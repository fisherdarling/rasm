use crate::runtime::runtime_store::{RuntimeStore, DefaultStore};
use crate::runtime::resolver::{Resolver, DefaultResolver};

// use std::rc::Rc;

#[derive(Debug)]
pub struct Runtime {
    store: Box<dyn RuntimeStore>, 
    resolver: Box<dyn Resolver>,
}

impl Runtime {
    pub fn default() -> Runtime {
        Self {
            store: Box::new(DefaultStore::default()),
            resolver: Box::new(DefaultResolver::default()),
        }
    }
}




