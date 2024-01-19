use super::{
    expr::Identifier,
    stmt::{Block, TypedParam},
    TypeS,
};
use base::located::Located;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub enum TopLevelDeclaration {
    Fun(Box<FunctionDeclaration>),
    TypeObject(Box<TypeObject>),
}

#[derive(Clone, Debug, Serialize)]
pub struct FunctionDeclaration {
    pub name: Located<Identifier>,
    pub params: Vec<TypedParam>,
    pub ret_ty: TypeS,
    pub body: Block,
}

impl FunctionDeclaration {
    pub fn new(
        name: Located<Identifier>,
        params: Vec<TypedParam>,
        ret_ty: TypeS,
        body: Block,
    ) -> FunctionDeclaration {
        FunctionDeclaration {
            name,
            params,
            ret_ty,
            body,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct TypeObject {
    pub name: Identifier,
    pub members: Vec<TypedParam>,
}
