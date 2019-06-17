use crate::function::FuncRef;

pub struct TableInst {
    refs: Vec<Option<FuncRef>>,
    max: Option<u32>,
}

impl TableInst {
    pub fn new(max: Option<u32>) -> Self {
        TableInst {
            refs: Vec::new(),
            max,
        }
    }
}
