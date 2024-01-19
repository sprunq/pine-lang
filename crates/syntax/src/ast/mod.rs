pub mod expr;
pub mod op;
pub mod stmt;
pub mod toplevel;
pub mod types;

use self::{toplevel::TopLevelDeclaration, types::Type};
use base::{located::Spanned, source_id::SourceId};
use expr::Expr;
use serde::Serialize;
use stmt::Stmt;

pub type StmtS = Spanned<Stmt>;
pub type ExprS = Spanned<Expr>;
pub type TopLevelS = Spanned<TopLevelDeclaration>;
pub type TypeS = Spanned<Type>;

#[derive(Clone, Debug, Serialize)]
pub struct Program {
    pub source: SourceId,
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
