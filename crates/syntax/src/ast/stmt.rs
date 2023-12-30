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

impl StmtBlock {
    pub fn new(stmts: Vec<StmtS>) -> Self {
        Self { stmts }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeObject {
    pub name: Identifier,
    pub members: Vec<TypedParam>,
}

impl TypeObject {
    pub fn new(name: Identifier, members: Vec<TypedParam>) -> Self {
        Self { name, members }
    }
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

impl StmtFun {
    pub fn new(name: Identifier, params: Vec<TypedParam>, ret_ty: TypeS, body: StmtBlock) -> Self {
        Self {
            name,
            params,
            ret_ty,
            body,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtIf {
    pub cond: ExprS,
    pub then: StmtS,
    pub else_: Option<StmtS>,
}

impl StmtIf {
    pub fn new(cond: ExprS, then: StmtS, else_: Option<StmtS>) -> Self {
        Self { cond, then, else_ }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtReturn {
    pub value: Option<ExprS>,
}

impl StmtReturn {
    pub fn new(value: Option<ExprS>) -> Self {
        Self { value }
    }

    pub fn void() -> Self {
        Self { value: None }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtBreak {
    pub span: Located<()>,
}

impl StmtBreak {
    pub fn new(span: Located<()>) -> Self {
        Self { span }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VariableDeclaration {
    pub var: Identifier,
    pub ty: TypeS,
    pub value: ExprS,
}

impl VariableDeclaration {
    pub fn new(var: Identifier, ty: TypeS, value: ExprS) -> Self {
        Self { var, ty, value }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtLoop {
    pub body: StmtS,
}

impl StmtLoop {
    pub fn new(body: StmtS) -> Self {
        Self { body }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtEmpty {
    pub span: Located<()>,
}

impl StmtEmpty {
    pub fn new(span: Located<()>) -> Self {
        Self { span }
    }
}
