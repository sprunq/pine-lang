use super::{expr::Identifier, impl_from_as_box, types::Type, ExprS, StmtS, TypeS};
use base::located::Located;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub enum Stmt {
    Block(Box<Block>),
    Expr(Box<ExprS>),
    If(Box<IfElse>),
    Return(Box<Return>),
    Break(Box<Break>),
    VariableDeclaration(Box<VariableDeclaration>),
    Loop(Box<Loop>),
    Empty(Box<Empty>),
    Assign(Box<Assign>),
}

impl_from_as_box!(Block => Stmt => Block);

#[derive(Clone, Debug, Serialize)]
pub struct Assign {
    pub var: ExprS,
    pub value: ExprS,
}

#[derive(Clone, Debug, Serialize)]
pub struct Block {
    pub stmts: Vec<StmtS>,
}

#[derive(Clone, Debug, Serialize)]
pub struct TypedParam {
    pub name: Located<Identifier>,
    pub ty: Located<Type>,
}

impl TypedParam {
    pub fn new(name: Located<Identifier>, ty: Located<Type>) -> TypedParam {
        TypedParam { name, ty }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct IfElse {
    pub cond: ExprS,
    pub then: StmtS,
    pub else_: Option<StmtS>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Return {
    pub value: Option<ExprS>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Break {
    pub span: Located<()>,
}

#[derive(Clone, Debug, Serialize)]
pub struct VariableDeclaration {
    pub var: Identifier,
    pub ty: TypeS,
    pub value: ExprS,
}

#[derive(Clone, Debug, Serialize)]
pub struct Loop {
    pub body: StmtS,
}

#[derive(Clone, Debug, Serialize)]
pub struct Empty {
    pub span: Located<()>,
}
