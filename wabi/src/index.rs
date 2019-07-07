pub use wasm_nom::types::index::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ModuleIdx(pub(crate) usize);

impl ModuleIdx {
    #[inline]
    pub fn index(&self) -> usize {
        self.0
    }

    #[inline]
    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }
}

impl std::ops::Deref for ModuleIdx {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ModuleIdx {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<usize> for ModuleIdx {
    fn from(val: usize) -> Self {
        Self(val)
    }
}

impl Into<usize> for ModuleIdx {
    fn into(self) -> usize {
        self.0
    }
}
