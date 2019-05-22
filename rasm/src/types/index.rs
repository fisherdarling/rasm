#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TypeIdx(u32);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FuncIdx(u32);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TableIdx(u32);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MemIdx(u32);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GlobalIdx(u32);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LocalIdx(u32);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LabelIdx(u32);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Index {
    Type(TypeIdx),
    Func(FuncIdx),
    Table(TableIdx),
    Mem(MemIdx),
    Global(GlobalIdx),
    Local(LocalIdx),
    Label(LabelIdx),
}
