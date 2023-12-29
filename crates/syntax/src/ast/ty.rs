use base::located::Located;

use super::expr::Identifier;

#[derive(Clone, Debug, PartialEq)]
pub struct TypedParam {
    pub name: Located<Identifier>,
    pub ty: Located<Type>,
}

#[derive(Clone, Debug, PartialEq)]
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
