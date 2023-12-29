use std::{path::PathBuf, process::Command};

use c_ast::{ast::*, ty::CType};

pub mod c_ast;
pub mod compiler_runner;
pub mod lib_core;
pub mod passes;

const INTERNAL_MAIN: &str = "pine_lang_main";
const KI_GC_NAME: &str = "gc";
const KI_GC_CLASS: &str = "GarbageCollector";
const KI_GC_NEW_CALL_SUFFIX: &str = "__internal__new_gc";
const KI_GC_NEW_CALL_PREFIX: &str = "_";

pub fn format_generated(source: PathBuf) {
    // run clang-format on generated source
    let mut cmd = Command::new("clang-format");
    cmd.arg("-i").arg(&source);
    cmd.arg("-style=file");
    cmd.output().expect("failed to format generated source");
}

pub fn build_c_main_file(internal_main_include_name: String) -> CTranslationUnit {
    // gc_start(&gc, &argc)
    let gc_start = CExpr::Call(CCallExpr::new(
        CExpr::Identifier(CIdentifier::new("gc_start")),
        vec![
            CExpr::Identifier(CIdentifier::new("&gc")),
            CExpr::Identifier(CIdentifier::new("&argc")),
        ],
    ));

    // internal_main()
    let main_call = CExpr::Call(CCallExpr::new(
        CExpr::Identifier(CIdentifier::new(INTERNAL_MAIN)),
        vec![],
    ));

    // gc_stop(&gc)
    let gc_stop = CExpr::Call(CCallExpr::new(
        CExpr::Identifier(CIdentifier::new("gc_stop")),
        vec![CExpr::Identifier(CIdentifier::new("&gc"))],
    ));

    let body = CBlockStmt::new(vec![gc_start.into(), main_call.into(), gc_stop.into()]).into();

    let main_fn: CDeclaration = CDeclaration::FunctionDeclaration(CFunctionDeclaration::new(
        CIdentifier::new("main"),
        vec![
            CTypedParam::new(CIdentifier::new("argc"), CType::I32),
            CTypedParam::new(
                CIdentifier::new("argv"),
                CType::Pointer(Box::new(CType::Pointer(Box::new(CType::I8)))),
            ),
        ],
        CType::I32,
        Some(body),
    ));

    let name = "main".into();
    let header_includes = vec![
        CHeaderInclude::new("pine_gc.h", false),
        CHeaderInclude::new("stdint.h", false),
        CHeaderInclude::new(format!("{}.h", internal_main_include_name), false),
    ];
    let implementation = vec![main_fn];
    CTranslationUnit {
        name,
        is_header: false,
        header_includes,
        implementation,
    }
}

#[allow(dead_code)]
fn build_gc_global_var() -> CDeclaration {
    CDeclaration::GlobalVariableDeclaration(CGlobalVariableDeclaration::new(
        CIdentifier::new("gc"),
        CType::Struct(KI_GC_CLASS.to_string()),
        None,
    ))
}
