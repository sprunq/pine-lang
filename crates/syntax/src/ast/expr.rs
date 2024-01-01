use std::fmt::{self, Display, Formatter};

use super::{
    op::{OpInfix, OpPrefix},
    ExprS,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Var(ExprVar),
    Literal(ExprLiteral),
    Call(Box<ExprCall>),
    MemberAccess(Box<ExprMemberAccess>),
    Prefix(Box<ExprPrefix>),
    Infix(Box<ExprInfix>),
    StructureInit(ExprStructureInit),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprStructureInit {
    pub name: Identifier,
    pub members: Vec<(Identifier, ExprS)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprCall {
    pub callee: ExprS,
    pub args: Vec<ExprS>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprMemberAccess {
    pub object: ExprS,
    pub member_name: Identifier,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprLiteral {
    Bool(bool),
    Nil,
    Integer(i64),
    Float(f64),
    String(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprInfix {
    pub lt: ExprS,
    pub op: OpInfix,
    pub rt: ExprS,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprPrefix {
    pub op: OpPrefix,
    pub rt: ExprS,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExprVar {
    pub var: Identifier,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Identifier {
    pub name: String,
}

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

impl From<Identifier> for String {
    fn from(val: Identifier) -> Self {
        val.name
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
