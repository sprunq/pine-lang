use base::located::Located;

use super::{expr::Identifier, ty::TypedParam, ExprS, StmtS, TypeS};

#[derive(Clone, Debug, PartialEq)]
pub enum Declaration {
    Fun(FunctionDeclaration),
    TypeObject(TypeObject),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Block(Block),
    Expr(StmtExpr),
    If(Box<IfElse>),
    Return(Return),
    Break(Break),
    VariableDeclaration(VariableDeclaration),
    Loop(Box<Loop>),
    Empty(Empty),
    Assign(Box<Assign>),
}

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
pub struct FunctionDeclaration {
    pub name: Identifier,
    pub params: Vec<TypedParam>,
    pub ret_ty: TypeS,
    pub body: Block,
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
