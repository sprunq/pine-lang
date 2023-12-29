use crate::c_ast::ast::*;
use crate::c_ast::op::{CAssignmentOperator, CBinaryOperator, CUnaryOperator};
use crate::c_ast::ty::CType;

pub struct CAstWriter {
    string: Vec<String>,
}

impl Default for CAstWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl CAstWriter {
    pub fn write_unit(unit: &CTranslationUnit) -> String {
        let mut writer = CAstWriter::new();
        writer.write_translation_unit(unit);
        writer.get_string()
    }

    pub fn new() -> Self {
        Self { string: Vec::new() }
    }

    pub fn write<S: AsRef<str>>(&mut self, s: S) {
        self.string.push(s.as_ref().to_string());
    }

    pub fn get_string(&self) -> String {
        let needed_capacity = self.string.iter().map(|s| s.len()).sum();
        let mut string = String::with_capacity(needed_capacity);
        for s in &self.string {
            string.push_str(s);
        }
        string
    }

    pub fn write_translation_unit(&mut self, tu: &CTranslationUnit) {
        for header_include in &tu.header_includes {
            self.write_header_include(header_include);
        }

        for decl in &tu.implementation {
            self.write_declaration(decl);
        }
    }

    fn write_header_include(&mut self, header_include: &CHeaderInclude) {
        self.write("#include");
        if header_include.is_system {
            self.write("<");
        } else {
            self.write("\"");
        }
        self.write(&header_include.name);
        if header_include.is_system {
            self.write(">");
        } else {
            self.write("\"");
        }
        self.write("\n");
    }

    fn write_stmt(&mut self, stmt: &CStmt) {
        match stmt {
            CStmt::Block(block) => self.write_block(block),
            CStmt::If(if_stmt) => self.write_if(if_stmt),
            CStmt::While(while_stmt) => self.write_while(while_stmt),
            CStmt::Return(return_stmt) => self.write_return(return_stmt),
            CStmt::Expr(expr_stmt) => self.write_expr_stmt(expr_stmt),
            CStmt::Continue => self.write("continue;"),
            CStmt::Break => self.write("break;"),
            CStmt::Empty => {}
            CStmt::VariableDeclaration(var_decl) => self.write_variable_declaration(var_decl),
        }
    }

    fn write_assignment(&mut self, ass: &CAssignment) {
        let op = self.get_assignment_op_token(&ass.op);
        self.write_expr(&ass.lhs);
        self.write(op);
        self.write_expr(&ass.rhs);
    }

    fn write_declaration(&mut self, decl: &CDeclaration) {
        match decl {
            CDeclaration::FunctionDeclaration(function_decl) => {
                self.write_function_declaration(function_decl);
            }
            CDeclaration::GlobalVariableDeclaration(var_decl) => {
                self.write_global_variable_declaration(var_decl);
            }
            CDeclaration::StructDeclaration(struct_decl) => {
                self.write_struct_declaration(struct_decl)
            }
        }
    }

    fn write_struct_declaration(&mut self, struct_decl: &CStructDeclaration) {
        self.write("typedef struct ");
        self.write_identifier(&struct_decl.name);
        self.write("{");

        for member in &struct_decl.members {
            if member.ty.points_to_struct() {
                self.write("struct ");
            }
            self.write_type(&member.ty);
            self.write(" ");
            self.write_identifier(&member.name);
            self.write(";");
        }

        self.write("}");
        self.write_identifier(&struct_decl.name);
        self.write(";");
    }

    fn write_function_declaration(&mut self, function_decl: &CFunctionDeclaration) {
        self.write_type(&function_decl.ret_ty);
        self.write(" ");
        self.write_identifier(&function_decl.name);
        self.write("(");
        for (i, param) in function_decl.params.iter().enumerate() {
            if i > 0 {
                self.write(", ");
            }
            self.write_typed_param(param);
        }
        self.write(")");
        if let Some(body) = &function_decl.body {
            self.write_stmt(body);
        } else {
            self.write(";");
        }
    }

    fn write_typed_param(&mut self, param: &CTypedParam) {
        self.write_type(&param.ty);
        self.write(" ");
        self.write_identifier(&param.name);
    }

    fn write_variable_declaration(&mut self, var_decl: &CVariableDeclaration) {
        self.write_type(&var_decl.ty);
        self.write(" ");
        self.write_identifier(&var_decl.name);
        self.write(";");
    }

    fn write_global_variable_declaration(&mut self, var_decl: &CGlobalVariableDeclaration) {
        self.write_type(&var_decl.ty);
        self.write(" ");
        self.write_identifier(&var_decl.name);
        if let Some(initializer) = &var_decl.initializer {
            self.write(" = ");
            self.write_expr(initializer);
        }
        self.write(";");
    }

    fn write_block(&mut self, block: &CBlockStmt) {
        self.write("{\n");
        for stmt in &block.stmts {
            self.write_stmt(stmt);
        }
        self.write("}\n");
    }

    fn write_if(&mut self, if_stmt: &CIfStmt) {
        self.write("if (");
        self.write_expr(&if_stmt.condition);
        self.write(") ");
        self.write_stmt(&if_stmt.then);
        if let Some(else_) = &if_stmt.else_ {
            self.write(" else ");
            self.write_stmt(else_);
        }
    }

    fn write_while(&mut self, while_stmt: &CWhileStmt) {
        self.write("while (");
        self.write_expr(&while_stmt.condition);
        self.write(") ");
        self.write_stmt(&while_stmt.body);
    }

    fn write_return(&mut self, return_stmt: &CReturnStmt) {
        self.write("return");
        if let Some(expr) = &return_stmt.expr {
            self.write(" ");
            self.write_expr(expr);
        }
        self.write(";");
    }

    fn write_expr_stmt(&mut self, expr_stmt: &CExpr) {
        self.write_expr(expr_stmt);
        self.write(";");
    }

    fn write_expr(&mut self, expr: &CExpr) {
        match expr {
            CExpr::Identifier(ident) => self.write(&ident.name),
            CExpr::Constant(constant) => self.write_constant(constant),
            CExpr::Member(member) => self.write_member(member),
            CExpr::Call(call) => self.write_call(call),
            CExpr::Cast(cast) => self.write_cast(cast),
            CExpr::Binary(binary) => self.write_binary(binary),
            CExpr::Unary(unary) => self.write_unary(unary),
            CExpr::SizeOf(sizeof) => self.write_sizeof(sizeof),
            CExpr::Assignment(ass) => self.write_assignment(ass),
            CExpr::Type(ty) => self.write_type_expr(ty),
        }
    }

    fn write_type_expr(&mut self, ty: &CType) {
        self.write_type(ty);
    }

    fn write_sizeof(&mut self, ty: &CType) {
        self.write("sizeof(");
        self.write_type(ty);
        self.write(")");
    }

    fn write_member(&mut self, member: &CMemberExpr) {
        self.write_expr(&member.expression);
        self.write_member_operator(&member.operator);
        self.write_identifier(&member.identifier);
    }

    fn write_member_operator(&mut self, member: &CMemberOperator) {
        match member {
            CMemberOperator::Direct => self.write("."),
            CMemberOperator::Indirect => self.write("->"),
        }
    }

    fn write_identifier(&mut self, ident: &CIdentifier) {
        self.write(&ident.name);
    }

    fn write_call(&mut self, call: &CCallExpr) {
        self.write_expr(&call.callee);
        self.write("(");
        for (i, arg) in call.args.iter().enumerate() {
            if i > 0 {
                self.write(", ");
            }
            self.write_expr(arg);
        }
        self.write(")");
    }

    fn write_cast(&mut self, cast: &CCastExpr) {
        self.write("(");
        self.write_type(&cast.ty);
        self.write(")");
        self.write_expr(&cast.expr);
    }

    fn write_binary(&mut self, binary: &CBinaryExpr) {
        let (left, right) = self.get_binary_op_token_left_right(&binary.operator);
        self.write_expr(&binary.left);
        self.write(left);
        self.write_expr(&binary.right);
        if let Some(right) = right {
            self.write(right);
        }
    }

    fn get_binary_op_token_left_right(
        &mut self,
        operator: &CBinaryOperator,
    ) -> (&'static str, Option<&'static str>) {
        match operator {
            CBinaryOperator::Index => ("[", Some("]")),
            CBinaryOperator::Multiply => ("*", None),
            CBinaryOperator::Divide => ("/", None),
            CBinaryOperator::Modulo => ("%", None),
            CBinaryOperator::Plus => ("+", None),
            CBinaryOperator::Minus => ("-", None),
            CBinaryOperator::ShiftLeft => ("<<", None),
            CBinaryOperator::ShiftRight => (">>", None),
            CBinaryOperator::Less => ("<", None),
            CBinaryOperator::Greater => (">", None),
            CBinaryOperator::LessOrEqual => ("<=", None),
            CBinaryOperator::GreaterOrEqual => (">=", None),
            CBinaryOperator::Equals => ("==", None),
            CBinaryOperator::NotEquals => ("!=", None),
            CBinaryOperator::BitwiseAnd => ("&", None),
            CBinaryOperator::BitwiseXor => ("^", None),
            CBinaryOperator::BitwiseOr => ("|", None),
            CBinaryOperator::LogicalAnd => ("&&", None),
            CBinaryOperator::LogicalOr => ("||", None),
        }
    }

    fn get_assignment_op_token(&mut self, operator: &CAssignmentOperator) -> &'static str {
        match operator {
            CAssignmentOperator::Assign => "=",
            CAssignmentOperator::AssignMultiply => "*=",
            CAssignmentOperator::AssignDivide => "/=",
            CAssignmentOperator::AssignModulo => "%=",
            CAssignmentOperator::AssignPlus => "+=",
            CAssignmentOperator::AssignMinus => "-=",
            CAssignmentOperator::AssignShiftLeft => "<<=",
            CAssignmentOperator::AssignShiftRight => ">>=",
            CAssignmentOperator::AssignBitwiseAnd => "&=",
            CAssignmentOperator::AssignBitwiseXor => "^=",
            CAssignmentOperator::AssignBitwiseOr => "|=",
        }
    }

    fn write_unary(&mut self, unary: &CUnaryExpr) {
        let (before, after) = self.get_unary_op_token_left_right(&unary.operator);
        if let Some(before) = before {
            self.write(before);
        }
        self.write_expr(&unary.expr);
        if let Some(after) = after {
            self.write(after);
        }
    }

    fn get_unary_op_token_left_right(
        &mut self,
        operator: &CUnaryOperator,
    ) -> (Option<&'static str>, Option<&'static str>) {
        match operator {
            CUnaryOperator::PostIncrement => (None, Some("++")),
            CUnaryOperator::PostDecrement => (None, Some("--")),
            CUnaryOperator::PreIncrement => (Some("++"), None),
            CUnaryOperator::PreDecrement => (Some("--"), None),
            CUnaryOperator::Address => (Some("&"), None),
            CUnaryOperator::Indirection => (Some("*"), None),
            CUnaryOperator::Plus => (Some("+"), None),
            CUnaryOperator::Minus => (Some("-"), None),
            CUnaryOperator::Complement => (Some("~"), None),
            CUnaryOperator::Negate => (Some("!"), None),
        }
    }

    fn write_constant(&mut self, constant: &CConstant) {
        match constant {
            CConstant::Integer(i) => self.write(&i.to_string()),
            CConstant::Float(f) => self.write(&f.to_string()),
            CConstant::String(s) => self.write(&format!("\"{}\"", s)),
        }
    }

    fn write_type(&mut self, ty: &CType) {
        match ty {
            CType::Void => self.write("void"),
            CType::U8 => self.write("uint8_t"),
            CType::U16 => self.write("uint16_t"),
            CType::U32 => self.write("uint32_t"),
            CType::U64 => self.write("uint64_t"),
            CType::I8 => self.write("int8_t"),
            CType::I16 => self.write("int16_t"),
            CType::I32 => self.write("int32_t"),
            CType::I64 => self.write("int64_t"),
            CType::F32 => self.write("float"),
            CType::F64 => self.write("double"),
            CType::String => self.write("char*"),
            CType::Struct(s) => self.write(s),
            CType::ISize => todo!(),
            CType::USize => todo!(),
            CType::Pointer(p) => {
                self.write_type(p);
                self.write("*");
            }
            CType::Reference(r) => {
                self.write("&");
                self.write_type(r);
            }
        }
    }
}
