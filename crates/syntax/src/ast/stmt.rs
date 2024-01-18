use super::{expr::Identifier, impl_from_as_box, types::Type, ExprS, StmtS, TypeS};
use base::located::Located;

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub struct Assign {
    pub var: ExprS,
    pub value: ExprS,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    pub stmts: Vec<StmtS>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypedParam {
    pub name: Located<Identifier>,
    pub ty: Located<Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IfElse {
    pub cond: ExprS,
    pub then: StmtS,
    pub else_: Option<StmtS>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Return {
    pub value: Option<ExprS>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Break {
    pub span: Located<()>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VariableDeclaration {
    pub var: Identifier,
    pub ty: TypeS,
    pub value: ExprS,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Loop {
    pub body: StmtS,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Empty {
    pub span: Located<()>,
}
