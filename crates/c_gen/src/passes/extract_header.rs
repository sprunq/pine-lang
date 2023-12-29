use crate::c_ast::ast::{CDeclaration, CFunctionDeclaration, CHeaderInclude, CTranslationUnit};

/// A utility struct for extracting header information from a C translation unit.
pub struct ExtractHeader {}

impl ExtractHeader {
    /// Extracts header information from the provided C translation unit.
    /// Every source file gets a matching header file since C cares about function ordering.
    /// Function declarations as well as struct declarations are stored in the header units.
    /// Struct declarations are removed from the source unit.
    pub fn extract(source: &mut CTranslationUnit) -> CTranslationUnit {
        let mut declarations = Vec::new();
        for decl in &source.implementation {
            match decl {
                CDeclaration::FunctionDeclaration(d) => {
                    declarations.push(CDeclaration::FunctionDeclaration(
                        Self::extract_header_from_function(d),
                    ));
                }
                CDeclaration::StructDeclaration(_) => {
                    declarations.push(decl.clone());
                }
                CDeclaration::GlobalVariableDeclaration(_) => {}
            }
        }

        // filter out structs
        source
            .implementation
            .retain(|decl| !matches!(decl, CDeclaration::StructDeclaration(_)));

        let includes = source.header_includes.clone();
        let name = source.name.clone();
        source.header_includes = vec![CHeaderInclude::new(format!("{}.h", name), false)];
        CTranslationUnit::new(name, true, includes, declarations)
    }

    fn extract_header_from_function(decl: &CFunctionDeclaration) -> CFunctionDeclaration {
        let name = decl.name.clone();
        let params = decl.params.clone();
        let ret_ty = decl.ret_ty.clone();

        CFunctionDeclaration {
            name,
            params,
            ret_ty,
            body: None,
        }
    }
}
