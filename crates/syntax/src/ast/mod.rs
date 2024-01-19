pub mod expr;
pub mod op;
pub mod stmt;
pub mod toplevel;
pub mod types;

use self::{toplevel::TopLevelDeclaration, types::Type};
use base::located::Located;
use expr::Expr;
use serde::Serialize;
use stmt::Stmt;

pub type StmtS = Located<Stmt>;
pub type ExprS = Located<Expr>;
pub type TopLevelS = Located<TopLevelDeclaration>;
pub type TypeS = Located<Type>;

#[derive(Clone, Debug, Serialize)]
pub struct Program {
    pub stmts: Vec<TopLevelS>,
}

macro_rules! impl_from_as_box {
    ($s:ty => $dst:ty => $expo:ident) => {
        impl From<$s> for $dst {
            fn from(src: $s) -> Self {
                <$dst>::$expo(Box::new(src))
            }
        }
    };
}

pub(crate) use impl_from_as_box;
