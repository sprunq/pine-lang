use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub enum Type {
    Unit,
    Bool,
    I8,
    I32,
    I64,
    U8,
    U32,
    U64,
    F32,
    F64,
    String,
    Struct(String),
}
