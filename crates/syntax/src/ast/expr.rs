use std::fmt::{self, Display, Formatter};

use super::{
    op::{OpInfix, OpPrefix},
    ExprS,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Var(ExprVar),
    Literal(ExprLiteral),
    Assign(Box<ExprAssign>),
    Call(Box<ExprCall>),
    MemberAccess(Box<ExprMemberAccess>),
    Prefix(Box<ExprPrefix>),
    Infix(Box<ExprInfix>),
    StructureInit(Box<ExprStructureInit>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprStructureInit {
    pub name: Identifier,
    pub members: Vec<(Identifier, ExprS)>,
}

impl ExprStructureInit {
    pub fn new(name: Identifier, members: Vec<(Identifier, ExprS)>) -> Self {
        Self { name, members }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprAssign {
    pub var: ExprS,
    pub value: ExprS,
}

impl ExprAssign {
    pub fn new(var: ExprS, value: ExprS) -> Self {
        Self { var, value }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprCall {
    pub callee: ExprS,
    pub args: Vec<ExprS>,
}

impl ExprCall {
    pub fn new(callee: ExprS, args: Vec<ExprS>) -> Self {
        Self { callee, args }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprMemberAccess {
    pub object: ExprS,
    pub member_name: Identifier,
}

impl ExprMemberAccess {
    pub fn new(object: ExprS, member_name: Identifier) -> Self {
        Self {
            object,
            member_name,
        }
    }
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

impl ExprInfix {
    pub fn new(lt: ExprS, op: OpInfix, rt: ExprS) -> Self {
        Self { lt, op, rt }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprPrefix {
    pub op: OpPrefix,
    pub rt: ExprS,
}

impl ExprPrefix {
    pub fn new(op: OpPrefix, rt: ExprS) -> Self {
        Self { op, rt }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExprVar {
    pub var: Identifier,
}

impl ExprVar {
    pub fn new(var: Identifier) -> Self {
        Self { var }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Identifier {
    pub name: String,
}

impl Identifier {
    pub fn new(name: String) -> Self {
        Self { name }
    }
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
