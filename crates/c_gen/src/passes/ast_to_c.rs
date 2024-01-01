use crate::c_ast::ast::*;
use crate::c_ast::op::{CAssignmentOperator, CBinaryOperator, CUnaryOperator};
use crate::c_ast::ty::CType;
use crate::{INTERNAL_MAIN, KI_GC_NAME, KI_GC_NEW_CALL_PREFIX, KI_GC_NEW_CALL_SUFFIX};
use syntax::ast::stmt::*;
use syntax::ast::ty::{Type, TypedParam};
use syntax::ast::ProgramUnit;
use syntax::ast::{expr::*, DeclS};
use syntax::*;

pub struct AstToCAst {
    // We need to keep track of the structs we've seen so we can generate the
    // new_gc functions for them since we need the type information.
    // TODO: make this better at some point
    seen_structs: Vec<CStructDeclaration>,
}

impl AstToCAst {
    pub fn new() -> Self {
        Self {
            seen_structs: Vec::new(),
        }
    }

    pub fn transform(program: &ProgramUnit, name: String) -> CTranslationUnit {
        let mut gen = Self::new();
        gen.build_translation_unit(program, name)
    }

    fn build_translation_unit(&mut self, program: &ProgramUnit, name: String) -> CTranslationUnit {
        let header_includes = self.include_headers();

        self.register_structs(program);

        // transform the declarations
        let decls = program
            .stmts
            .iter()
            .map(|stmt| self.build_declaration(&stmt.value))
            .collect::<Vec<_>>();

        // the init functions for the structs
        let struct_inits = self.build_new_gc_struct_inits(&program.stmts);

        let declarations = decls.into_iter().chain(struct_inits).collect::<Vec<_>>();

        CTranslationUnit {
            name,
            is_header: false,
            header_includes,
            implementation: declarations,
        }
    }

    fn register_structs(&mut self, program: &ProgramUnit) {
        for stmt in &program.stmts {
            if let Declaration::TypeObject(s) = &stmt.value {
                let name = CIdentifier::new(&s.name);
                let mut members = Vec::new();
                for member in &s.members {
                    let ty = self.build_ty(&member.ty.value);
                    let name = CIdentifier::new(&member.name.value);
                    members.push(CTypedParam::new(name, ty));
                }
                let s = CStructDeclaration::new(name, members);
                self.seen_structs.push(s);
            }
        }
    }

    fn find_struct_decl(&self, name: &CIdentifier) -> Option<&CStructDeclaration> {
        self.seen_structs.iter().find(|s| s.name == *name)
    }

    fn include_headers(&mut self) -> Vec<CHeaderInclude> {
        vec![
            CHeaderInclude::new("stdint.h", true),
            CHeaderInclude::new("pine_gc.h", false),
            CHeaderInclude::new("pine_io.h", false),
        ]
    }

    fn build_new_gc_struct_inits(&mut self, decls: &[DeclS]) -> Vec<CDeclaration> {
        let statements = decls
            .iter()
            .filter_map(|s| {
                if let Declaration::TypeObject(s) = &s.value {
                    let new_gc = self.build_struct_new_gc(s);
                    Some(new_gc.into())
                } else {
                    None
                }
            })
            .collect::<Vec<CDeclaration>>();
        statements
    }

    fn build_declaration(&mut self, decl: &Declaration) -> CDeclaration {
        match decl {
            Declaration::Fun(f) => {
                CDeclaration::FunctionDeclaration(self.build_function_declaration(f))
            }
            Declaration::TypeObject(s) => self.build_struct_declaration(s).into(),
        }
    }

    fn build_stmt(&mut self, stmt: &Stmt) -> Vec<CStmt> {
        match stmt {
            Stmt::Block(s) => vec![self.build_block(s).into()],
            Stmt::Expr(s) => vec![self.emit_expr_stmt(s)],
            Stmt::If(s) => vec![self.build_if(s).into()],
            Stmt::Return(s) => vec![self.build_return(s).into()],
            Stmt::Break(_) => vec![CStmt::Break],
            Stmt::Loop(s) => vec![self.build_loop(s).into()],
            Stmt::Empty(_) => vec![CStmt::Empty],
            Stmt::VariableDeclaration(let_stmt) => self.build_variable_declaration(let_stmt),
            Stmt::Error => unreachable!(),
        }
    }

    fn build_struct_declaration(&mut self, s: &TypeObject) -> CStructDeclaration {
        let name = CIdentifier::new(&s.name);
        let mut members = Vec::new();
        for member in &s.members {
            let ty = self.build_ty(&member.ty.value);
            let name = CIdentifier::new(&member.name.value);
            members.push(CTypedParam::new(name, ty));
        }
        CStructDeclaration::new(name, members)
    }

    /// Alpha *alpha_new_gc(int a, int b)
    /// {
    ///    Alpha *_newobj_alloc = (Alpha *)gc_malloc(&gc, sizeof(Alpha));
    ///    _newobj_alloc->a = a;
    ///    _newobj_alloc->b = b;
    ///    return _newobj_alloc;
    /// }
    fn build_struct_new_gc(&mut self, s: &TypeObject) -> CFunctionDeclaration {
        let mut block = vec![];
        let return_ty = CType::Pointer(Box::new(CType::Struct(s.name.to_string())));
        let callee = CIdentifier::new(Self::gc_constructor_call(&s.name));
        let params = s
            .members
            .iter()
            .map(|m| self.build_typed_param(m))
            .collect();
        let alloc_obj_ident = CIdentifier::new("n");
        let new_gc_alloc_call = self.build_casted_gc_alloc_call(&CIdentifier::new(&s.name));

        let decl: CStmt =
            CVariableDeclaration::new(alloc_obj_ident.clone(), return_ty.clone()).into();
        block.push(decl);

        let assignment: CExpr = CAssignment::new(
            alloc_obj_ident.clone().into(),
            CAssignmentOperator::Assign,
            new_gc_alloc_call.into(),
        )
        .into();
        let assignment = assignment.into();
        block.push(assignment);

        for ty_p in &s.members {
            let name = self.build_identifier(&ty_p.name.value);
            let member = CMemberExpr::new(
                alloc_obj_ident.clone().into(),
                CMemberOperator::Indirect,
                name,
            );
            let member = member.into();
            let value = CExpr::Identifier(self.build_identifier(&ty_p.name.value));
            let assignment = CAssignment::new(member, CAssignmentOperator::Assign, value);
            let assignment = CStmt::Expr(assignment.into());
            block.push(assignment);
        }

        let return_: CStmt = CReturnStmt::new(Some(alloc_obj_ident.clone().into())).into();
        block.push(return_);

        let body = CBlockStmt::new(block).into();
        CFunctionDeclaration::new(callee, params, return_ty, Some(body))
    }

    /// (Alpha *)gc_malloc(&gc, sizeof(Alpha))
    fn build_casted_gc_alloc_call(&mut self, s: &CIdentifier) -> CCastExpr {
        let new_gc_alloc_call = self.build_new_gc_alloc_call(&CIdentifier::new(&s.name));

        CCastExpr::new(
            CType::Pointer(Box::new(CType::Struct(s.name.to_string()))),
            new_gc_alloc_call.into(),
        )
    }

    ///  gc_malloc(&gc, sizeof(Alpha))
    fn build_new_gc_alloc_call(&mut self, s: &CIdentifier) -> CCallExpr {
        let gc_malloc = CIdentifier::new("gc_malloc").into();
        let gc = CExpr::Type(CType::Reference(Box::new(CType::Struct(
            KI_GC_NAME.to_string(),
        ))));
        let size_of = CExpr::SizeOf(CType::Struct(s.name.clone()));
        CCallExpr::new(gc_malloc, vec![gc, size_of])
    }

    fn build_typed_param(&mut self, param: &TypedParam) -> CTypedParam {
        let ty = self.build_ty(&param.ty.value);
        let name = CIdentifier::new(&param.name.value);
        CTypedParam::new(name, ty)
    }

    fn build_block(&mut self, block: &StmtBlock) -> CBlockStmt {
        let mut stmts = Vec::new();
        for stmt in &block.stmts {
            let s = self.build_stmt(&stmt.value);
            stmts.extend(s);
        }
        CBlockStmt::new(stmts)
    }

    fn build_function_declaration(&mut self, fun: &StmtFun) -> CFunctionDeclaration {
        let name = if fun.name.as_ref() == "main" {
            INTERNAL_MAIN.to_string()
        } else {
            fun.name.name.to_string()
        };
        let name = CIdentifier::new(name);
        let ret_ty = self.build_ty(&fun.ret_ty.value);
        let params = self.build_function_params(&fun.params);
        let body = Some(self.build_block(&fun.body)).map(|b| b.into());
        CFunctionDeclaration::new(name, params, ret_ty, body)
    }

    fn build_function_params(&mut self, params: &Vec<TypedParam>) -> Vec<CTypedParam> {
        let mut c_params = Vec::new();
        for param in params {
            let ident = CIdentifier::new(&param.name.value);
            let ty = self.build_ty(&param.ty.value);
            c_params.push(CTypedParam::new(ident, ty));
        }
        c_params
    }

    fn build_if(&mut self, if_stmt: &StmtIf) -> CIfStmt {
        let cond = self.build_expr(&if_stmt.cond.value);
        let then = self.build_stmt_expect_block(&if_stmt.then.value).into();
        let mut else_ = None;
        if let Some(els) = &if_stmt.else_ {
            let b = self.build_stmt_expect_block(&els.value);
            else_ = Some(b.into());
        }
        CIfStmt::new(cond, then, else_)
    }

    // build_stmt returns a vec of CStmts. But we know that this is a block
    fn build_stmt_expect_block(&mut self, stmt: &Stmt) -> CBlockStmt {
        match stmt {
            Stmt::Block(b) => self.build_block(b),
            _ => unreachable!("Loop body must be a block"),
        }
    }

    fn build_return(&mut self, return_stmt: &StmtReturn) -> CReturnStmt {
        if let Some(value) = &return_stmt.value {
            let expr = self.build_expr(&value.value);
            CReturnStmt::new(Some(expr))
        } else {
            CReturnStmt::new(None)
        }
    }

    /// Input:
    ///     let a: Alpha = Alpha { a: 1, b: 2 };
    /// Output:
    ///     Alpha *a;
    ///     {
    ///         int _a;
    ///         _a = 1;
    ///         int _b;
    ///         _b = 2;
    ///         a = alpha_new_gc(_a, _b);
    ///     }
    fn build_structure_init(
        &mut self,
        assign_to_var: &CIdentifier,
        si: &ExprStructureInit,
    ) -> Vec<CStmt> {
        // Alpha *x;
        let struct_name = self.build_identifier(&si.name);
        let structure = self.find_struct_decl(&struct_name).unwrap().clone();
        let name = structure.name.clone();
        let ty = Self::pointer_to_struct(&name);
        let decl: CStmt = CVariableDeclaration::new(assign_to_var.clone(), ty).into();

        let mut inner_block_stmts = Vec::new();
        let mut arg_idents = Vec::new();
        for (ident, value) in &si.members {
            let value = &value.value;
            let init_name = self.build_identifier(ident);
            let init_ty = structure
                .members
                .iter()
                .find(|m| m.name == init_name)
                .unwrap()
                .ty
                .clone();
            let init_decl = CVariableDeclaration::new(init_name.clone(), init_ty);
            inner_block_stmts.push(init_decl.into());

            let init_value = self.build_expr(value);
            let init_assignment = CAssignment::new(
                init_name.clone().into(),
                CAssignmentOperator::Assign,
                init_value,
            );
            inner_block_stmts.push(CStmt::Expr(init_assignment.into()));
            arg_idents.push(init_name.into());
        }

        // a = alpha_new_gc(1, 2);
        let call = Self::build_struct_new_call(name.as_ref(), arg_idents);
        let assignment = CAssignment::new(
            assign_to_var.clone().into(),
            CAssignmentOperator::Assign,
            call.into(),
        );
        inner_block_stmts.push(CStmt::Expr(assignment.into()));
        let inner_block: CStmt = CBlockStmt::new(inner_block_stmts).into();
        vec![decl, inner_block]
    }

    /// int64_t x;
    /// x = 1;
    fn build_variable_declaration(&mut self, let_stmt: &VariableDeclaration) -> Vec<CStmt> {
        if let Expr::StructureInit(si) = &let_stmt.value.value {
            let name = self.build_identifier(&let_stmt.var);
            return self.build_structure_init(&name, si);
        }

        let name = CIdentifier::new(&let_stmt.var.name);
        let ty = self.build_ty(&let_stmt.ty.value);
        let decl = CVariableDeclaration::new(name.clone(), ty).into();
        let value = self.build_expr(&let_stmt.value.value);
        let assignment = CAssignment::new(name.into(), CAssignmentOperator::Assign, value);
        let assignment = CStmt::Expr(assignment.into());
        vec![decl, assignment]
    }

    fn build_loop(&mut self, loop_stmt: &StmtLoop) -> CWhileStmt {
        let cond = CExpr::Constant(CConstant::Integer(1));
        match &loop_stmt.body.value {
            Stmt::Block(b) => {
                let body = self.build_block(b).into();
                CWhileStmt::new(cond, body)
            }
            _ => unreachable!("Loop body must be a block"),
        }
    }

    fn emit_expr_stmt(&mut self, expr_stmt: &StmtExpr) -> CStmt {
        let expr = self.build_expr(&expr_stmt.value.value);
        CStmt::Expr(expr)
    }

    fn build_expr(&mut self, expr: &Expr) -> CExpr {
        match expr {
            Expr::Var(e) => self.build_var(e).into(),
            Expr::Literal(e) => self.build_literal(e).into(),
            Expr::Assign(e) => self.build_assign(e).into(),
            Expr::Call(e) => self.build_call(e).into(),
            Expr::Prefix(e) => self.build_prefix(e).into(),
            Expr::Infix(e) => self.build_infix(e).into(),
            Expr::MemberAccess(e) => self.build_get(e).into(),
            Expr::StructureInit(_) => unreachable!(),
        }
    }

    fn build_var(&mut self, var: &ast::expr::ExprVar) -> CIdentifier {
        self.build_identifier(&var.var)
    }

    fn build_identifier(&mut self, var: &ast::expr::Identifier) -> CIdentifier {
        var.name.to_string().into()
    }

    fn build_literal(&mut self, literal: &ast::expr::ExprLiteral) -> CConstant {
        match literal {
            ast::expr::ExprLiteral::Bool(b) => {
                if *b {
                    CConstant::Integer(1)
                } else {
                    CConstant::Integer(0)
                }
            }
            ast::expr::ExprLiteral::Integer(i) => CConstant::Integer(*i),
            ast::expr::ExprLiteral::Float(f) => CConstant::Float(*f),
            ast::expr::ExprLiteral::String(_s) => todo!(),
            ast::expr::ExprLiteral::Nil => todo!(),
        }
    }

    fn build_assign(&mut self, assign: &ast::expr::ExprAssign) -> CAssignment {
        let ident = self.build_expr(&assign.var.value);
        let ass = CAssignmentOperator::Assign;
        let value = self.build_expr(&assign.value.value);
        CAssignment::new(ident, ass, value)
    }

    fn build_call(&mut self, call: &ast::expr::ExprCall) -> CCallExpr {
        let callee = self.build_expr(&call.callee.value);
        let mut args = Vec::new();
        for argument in &call.args {
            let arg = self.build_expr(&argument.value);
            args.push(arg);
        }
        CCallExpr::new(callee, args)
    }

    // a->b
    fn build_get(&mut self, get: &ast::expr::ExprMemberAccess) -> CMemberExpr {
        let obj = self.build_expr(&get.object.value);
        let ident = self.build_identifier(&get.member_name);
        CMemberExpr::new(obj, CMemberOperator::Indirect, ident)
    }

    fn build_prefix(&mut self, prefix: &ast::expr::ExprPrefix) -> CUnaryExpr {
        let operator = match prefix.op {
            ast::op::OpPrefix::Negate => CUnaryOperator::Negate,
            ast::op::OpPrefix::Not => CUnaryOperator::Complement,
        };
        let expr = self.build_expr(&prefix.rt.value);
        CUnaryExpr::new(operator, expr)
    }

    fn build_infix(&mut self, infix: &ast::expr::ExprInfix) -> CBinaryExpr {
        let left = self.build_expr(&infix.lt.value);
        let operator = self.build_binary_op(&infix.op);
        let right = self.build_expr(&infix.rt.value);
        CBinaryExpr::new(operator, left, right)
    }

    fn build_binary_op(&self, op: &ast::op::OpInfix) -> CBinaryOperator {
        match op {
            ast::op::OpInfix::Add => CBinaryOperator::Plus,
            ast::op::OpInfix::Subtract => CBinaryOperator::Minus,
            ast::op::OpInfix::Multiply => CBinaryOperator::Multiply,
            ast::op::OpInfix::Divide => CBinaryOperator::Divide,
            ast::op::OpInfix::Modulo => CBinaryOperator::Modulo,
            ast::op::OpInfix::Less => CBinaryOperator::Less,
            ast::op::OpInfix::LessEqual => CBinaryOperator::LessOrEqual,
            ast::op::OpInfix::Greater => CBinaryOperator::Greater,
            ast::op::OpInfix::GreaterEqual => CBinaryOperator::GreaterOrEqual,
            ast::op::OpInfix::Equal => CBinaryOperator::Equals,
            ast::op::OpInfix::NotEqual => CBinaryOperator::NotEquals,
            ast::op::OpInfix::LogicAnd => CBinaryOperator::LogicalAnd,
            ast::op::OpInfix::LogicOr => CBinaryOperator::LogicalOr,
        }
    }

    fn build_ty(&mut self, ty: &ast::ty::Type) -> CType {
        match ty {
            Type::Unit => CType::Void,
            Type::Bool => CType::U8,
            Type::U8 => CType::U8,
            Type::U32 => CType::U32,
            Type::U64 => CType::U64,
            Type::I8 => CType::I8,
            Type::I32 => CType::I32,
            Type::I64 => CType::I64,
            Type::F32 => CType::F32,
            Type::F64 => CType::F64,
            Type::String => todo!(),
            // We always pass structs by pointer
            Type::Struct(s) => CType::Pointer(Box::new(CType::Struct(s.to_string()))),
        }
    }

    /// alpha_new_gc(a,b)
    fn build_struct_new_call(s: &str, args: Vec<CExpr>) -> CCallExpr {
        let callee = CIdentifier::new(Self::gc_constructor_call(s)).into();
        CCallExpr::new(callee, args)
    }

    fn gc_constructor_call<S: AsRef<str>>(s: S) -> String {
        format!(
            "{}{}{}",
            KI_GC_NEW_CALL_PREFIX,
            s.as_ref(),
            KI_GC_NEW_CALL_SUFFIX
        )
    }

    fn pointer_to_struct<S: AsRef<str>>(ty: S) -> CType {
        let ty = CType::Struct(ty.as_ref().to_string());
        CType::Pointer(Box::new(ty))
    }
}

impl Default for AstToCAst {
    fn default() -> Self {
        Self::new()
    }
}
