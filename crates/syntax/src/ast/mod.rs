pub mod expr;
pub mod op;
pub mod stmt;
pub mod ty;

use self::{stmt::Declaration, ty::Type};
use base::located::Located;
use expr::Expr;
use stmt::Stmt;

pub type StmtS = Located<Stmt>;
pub type ExprS = Located<Expr>;
pub type DeclS = Located<Declaration>;
pub type TypeS = Located<Type>;

#[derive(Debug, Default)]
pub struct Program {
    pub stmts: Vec<DeclS>,
}
