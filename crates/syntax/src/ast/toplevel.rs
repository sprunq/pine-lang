use super::{
    expr::Identifier,
    stmt::{Block, TypedParam},
    TypeS,
};

#[derive(Clone, Debug, PartialEq)]
pub enum TopLevelDeclaration {
    Fun(Box<FunctionDeclaration>),
    TypeObject(Box<TypeObject>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionDeclaration {
    pub name: Identifier,
    pub params: Vec<TypedParam>,
    pub ret_ty: TypeS,
    pub body: Block,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeObject {
    pub name: Identifier,
    pub members: Vec<TypedParam>,
}
