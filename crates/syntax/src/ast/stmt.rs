use base::located::Located;

use super::{expr::Identifier, ty::TypedParam, ExprS, StmtS, TypeS};

#[derive(Clone, Debug, PartialEq)]
pub enum Declaration {
    Fun(StmtFun),
    TypeObject(TypeObject),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Block(StmtBlock),
    Expr(StmtExpr),
    If(Box<StmtIf>),
    Return(StmtReturn),
    Break(StmtBreak),
    VariableDeclaration(VariableDeclaration),
    Loop(Box<StmtLoop>),
    Empty(StmtEmpty),
    Error,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtBlock {
    pub stmts: Vec<StmtS>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeObject {
    pub name: Identifier,
    pub members: Vec<TypedParam>,
}

/// An expression statement evaluates an expression and discards the result.
#[derive(Clone, Debug, PartialEq)]
pub struct StmtExpr {
    pub value: ExprS,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtFun {
    pub name: Identifier,
    pub params: Vec<TypedParam>,
    pub ret_ty: TypeS,
    pub body: StmtBlock,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtIf {
    pub cond: ExprS,
    pub then: StmtS,
    pub else_: Option<StmtS>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtReturn {
    pub value: Option<ExprS>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtBreak {
    pub span: Located<()>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VariableDeclaration {
    pub var: Identifier,
    pub ty: TypeS,
    pub value: ExprS,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtLoop {
    pub body: StmtS,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtEmpty {
    pub span: Located<()>,
}
