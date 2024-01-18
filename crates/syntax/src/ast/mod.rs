pub mod expr;
pub mod op;
pub mod stmt;
pub mod toplevel;
pub mod types;

use self::{toplevel::TopLevelDeclaration, types::Type};
use base::located::Located;
use expr::Expr;
use stmt::Stmt;

pub type StmtS = Located<Stmt>;
pub type ExprS = Located<Expr>;
pub type DeclS = Located<TopLevelDeclaration>;
pub type TypeS = Located<Type>;

#[derive(Debug, Default)]
pub struct Program {
    pub stmts: Vec<DeclS>,
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
