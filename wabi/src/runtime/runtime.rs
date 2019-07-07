use crate::store::Store;
use crate::runtime::module_instance::ModuleInstance;

pub struct Runtime {
    store: Store,
    modules: Vec<ModuleInstance>,
    resolver: Resolver,
}