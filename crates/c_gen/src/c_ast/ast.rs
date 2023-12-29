use crate::c_ast::op::{CAssignmentOperator, CBinaryOperator, CUnaryOperator};
use crate::c_ast::ty::CType;

macro_rules! impl_from {
    ($s:ty => $dst:ty => $expo:ident) => {
        impl From<$s> for $dst {
            fn from(src: $s) -> Self {
                <$dst>::$expo(src)
            }
        }
    };
}

#[derive(Debug, PartialEq, Clone)]
pub struct CIdentifier {
    pub name: String,
}
impl CIdentifier {
    pub fn new<T: AsRef<str>>(name: T) -> Self {
        Self {
            name: name.as_ref().to_string(),
        }
    }
}

impl AsRef<str> for CIdentifier {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CConstant {
    Integer(i64),
    Float(f64),
    String(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct CTranslationBundle {
    pub header: CTranslationUnit,
    pub source: CTranslationUnit,
}

impl CTranslationBundle {
    pub fn new(header: CTranslationUnit, source: CTranslationUnit) -> Self {
        Self { header, source }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CTranslationUnit {
    pub name: String,
    pub is_header: bool,
    pub header_includes: Vec<CHeaderInclude>,
    pub implementation: Vec<CDeclaration>,
}

impl CTranslationUnit {
    pub fn new(
        name: String,
        is_header: bool,
        header_includes: Vec<CHeaderInclude>,
        declarations: Vec<CDeclaration>,
    ) -> Self {
        Self {
            name,
            is_header,
            header_includes,
            implementation: declarations,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CDeclaration {
    FunctionDeclaration(CFunctionDeclaration),
    GlobalVariableDeclaration(CGlobalVariableDeclaration),
    StructDeclaration(CStructDeclaration),
}

impl_from!(CGlobalVariableDeclaration => CDeclaration => GlobalVariableDeclaration);
impl_from!(CFunctionDeclaration => CDeclaration => FunctionDeclaration);
impl_from!(CStructDeclaration => CDeclaration => StructDeclaration);

#[derive(Debug, PartialEq, Clone)]
pub enum CStmt {
    Empty,
    Continue,
    Break,
    Return(CReturnStmt),
    Block(CBlockStmt),
    If(CIfStmt),
    While(CWhileStmt),
    VariableDeclaration(CVariableDeclaration),
    Expr(CExpr),
}

impl_from!(CReturnStmt => CStmt => Return);
impl_from!(CBlockStmt => CStmt => Block);
impl_from!(CIfStmt => CStmt => If);
impl_from!(CWhileStmt => CStmt => While);
impl_from!(CExpr => CStmt => Expr);
impl_from!(CVariableDeclaration => CStmt => VariableDeclaration);

#[derive(Debug, PartialEq, Clone)]
pub enum CExpr {
    Identifier(CIdentifier),
    Constant(CConstant),
    Member(CMemberExpr),
    Call(CCallExpr),
    Cast(CCastExpr),
    Binary(CBinaryExpr),
    Unary(CUnaryExpr),
    SizeOf(CType),
    Assignment(CAssignment),
    Type(CType),
}

impl_from!(CIdentifier => CExpr => Identifier);
impl_from!(CConstant => CExpr => Constant);
impl_from!(CType => CExpr => SizeOf);
impl_from!(CMemberExpr => CExpr => Member);
impl_from!(CCallExpr => CExpr => Call);
impl_from!(CCastExpr => CExpr => Cast);
impl_from!(CBinaryExpr => CExpr => Binary);
impl_from!(CUnaryExpr => CExpr => Unary);
impl_from!(CAssignment => CExpr => Assignment);

#[derive(Debug, PartialEq, Clone)]
pub struct CAssignment {
    pub lhs: Box<CExpr>,
    pub op: CAssignmentOperator,
    pub rhs: Box<CExpr>,
}

impl CAssignment {
    pub fn new(lhs: CExpr, op: CAssignmentOperator, rhs: CExpr) -> Self {
        Self {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CStructDeclaration {
    pub name: CIdentifier,
    pub members: Vec<CTypedParam>,
}

impl CStructDeclaration {
    pub fn new(name: CIdentifier, members: Vec<CTypedParam>) -> Self {
        Self { name, members }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CHeaderInclude {
    pub name: String,
    /// Whether to use `#include <…>` or `#include "…"`
    pub is_system: bool,
}

impl CHeaderInclude {
    pub fn new<S: AsRef<str>>(name: S, is_system: bool) -> Self {
        Self {
            name: name.as_ref().to_string(),
            is_system,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CGlobalVariableDeclaration {
    pub name: CIdentifier,
    pub ty: CType,
    pub initializer: Option<CExpr>,
}

impl CGlobalVariableDeclaration {
    pub fn new(name: CIdentifier, ty: CType, initializer: Option<CExpr>) -> Self {
        Self {
            name,
            ty,
            initializer,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CVariableDeclaration {
    pub name: CIdentifier,
    pub ty: CType,
}

impl CVariableDeclaration {
    pub fn new(name: CIdentifier, ty: CType) -> Self {
        Self { name, ty }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CFunctionDeclaration {
    pub name: CIdentifier,
    pub params: Vec<CTypedParam>,
    pub ret_ty: CType,
    pub body: Option<CStmt>,
}

impl CFunctionDeclaration {
    pub fn new(
        name: CIdentifier,
        params: Vec<CTypedParam>,
        ret_ty: CType,
        body: Option<CStmt>,
    ) -> Self {
        Self {
            name,
            params,
            ret_ty,
            body,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CTypedParam {
    pub name: CIdentifier,
    pub ty: CType,
}

impl CTypedParam {
    pub fn new(name: CIdentifier, ty: CType) -> Self {
        Self { name, ty }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CCallExpr {
    pub callee: Box<CExpr>,
    pub args: Vec<CExpr>,
}

impl CCallExpr {
    pub fn new(callee: CExpr, args: Vec<CExpr>) -> Self {
        Self {
            callee: Box::new(callee),
            args,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CCastExpr {
    pub ty: CType,
    pub expr: Box<CExpr>,
}

impl CCastExpr {
    pub fn new(ty: CType, expr: CExpr) -> Self {
        Self {
            ty,
            expr: Box::new(expr),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CBinaryExpr {
    pub operator: CBinaryOperator,
    pub left: Box<CExpr>,
    pub right: Box<CExpr>,
}

impl CBinaryExpr {
    pub fn new(operator: CBinaryOperator, left: CExpr, right: CExpr) -> Self {
        Self {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CUnaryExpr {
    pub operator: CUnaryOperator,
    pub expr: Box<CExpr>,
}

impl CUnaryExpr {
    pub fn new(operator: CUnaryOperator, expr: CExpr) -> Self {
        Self {
            operator,
            expr: Box::new(expr),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CMemberExpr {
    pub operator: CMemberOperator,
    pub expression: Box<CExpr>,
    pub identifier: CIdentifier,
}

impl CMemberExpr {
    pub fn new(expression: CExpr, operator: CMemberOperator, identifier: CIdentifier) -> Self {
        Self {
            operator,
            expression: Box::new(expression),
            identifier,
        }
    }
}

/// Struct or union member access
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum CMemberOperator {
    /// `a.b`
    Direct,
    /// `a->b`
    Indirect,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CReturnStmt {
    pub expr: Option<Box<CExpr>>,
}

impl CReturnStmt {
    pub fn new(expr: Option<CExpr>) -> Self {
        Self {
            expr: expr.map(Box::new),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CIfStmt {
    pub condition: CExpr,
    pub then: Box<CStmt>,
    pub else_: Option<Box<CStmt>>,
}

impl CIfStmt {
    pub fn new(cond: CExpr, then: CStmt, else_: Option<CStmt>) -> Self {
        Self {
            condition: cond,
            then: Box::new(then),
            else_: else_.map(Box::new),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CBlockStmt {
    pub stmts: Vec<CStmt>,
}

impl CBlockStmt {
    pub fn new(stmts: Vec<CStmt>) -> Self {
        Self { stmts }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CWhileStmt {
    pub condition: CExpr,
    pub body: Box<CStmt>,
}

impl CWhileStmt {
    pub fn new(cond: CExpr, body: CStmt) -> Self {
        Self {
            condition: cond,
            body: Box::new(body),
        }
    }
}

impl From<String> for CIdentifier {
    fn from(name: String) -> Self {
        Self { name }
    }
}

impl From<&str> for CIdentifier {
    fn from(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
