use super::{
    op::{OpInfix, OpPrefix},
    ExprS,
};
use serde::Serialize;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, Serialize)]
pub enum Expr {
    Variable(Box<Variable>),
    Literal(Box<Literal>),
    Call(Box<Call>),
    MemberAccess(Box<MemberAccess>),
    Prefix(Box<Prefix>),
    Infix(Box<Infix>),
    StructureInit(Box<StructureInit>),
}

#[derive(Clone, Debug, Serialize)]
pub struct StructureInit {
    pub name: Identifier,
    pub members: Vec<(Identifier, ExprS)>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Call {
    pub callee: ExprS,
    pub args: Vec<ExprS>,
}

#[derive(Clone, Debug, Serialize)]
pub struct MemberAccess {
    pub object: ExprS,
    pub member_name: Identifier,
}

#[derive(Clone, Debug, Serialize)]
pub enum Literal {
    Bool(bool),
    Nil,
    Integer(i64),
    Float(f64),
    String(String),
}

#[derive(Clone, Debug, Serialize)]
pub struct Infix {
    pub lt: ExprS,
    pub op: OpInfix,
    pub rt: ExprS,
}

#[derive(Clone, Debug, Serialize)]
pub struct Prefix {
    pub op: OpPrefix,
    pub rt: ExprS,
}

#[derive(Clone, Debug, Serialize)]
pub struct Variable {
    pub var: Identifier,
}

#[derive(Clone, Debug, Serialize)]
pub struct Identifier {
    pub name: String,
}

impl Identifier {
    pub fn new<S: AsRef<str>>(name: S) -> Identifier {
        Identifier {
            name: name.as_ref().to_string(),
        }
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

impl From<&str> for Identifier {
    fn from(val: &str) -> Self {
        Identifier {
            name: val.to_string(),
        }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
