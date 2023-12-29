// pub mod rc_pass;

use std::{
    path::{Path, PathBuf},
    sync::mpsc::Sender,
};

use base::{compile_context::CompileContext, source_id::SourceId};
use c_gen::{
    c_ast::{ast::CTranslationUnit, write::CAstWriter},
    compiler_runner::{gcc::Gcc, CodegenRunner, OptLevel, RunnerOptions},
    lib_core::copy_core_c,
    passes::{ast_to_c::AstToCAst, extract_header::ExtractHeader},
};
use messages::message::Message;
use syntax::{ast::Program, parser::Parser};

pub struct Compiler<'a> {
    context: &'a mut CompileContext,
}

impl<'a> Compiler<'a> {
    pub fn new(context: &'a mut CompileContext) -> Self {
        Self { context }
    }

    pub fn compile(&mut self, message_sender: Sender<Message>) {
        let source_id = SourceId::from_path(&self.context.build_pkg);

        let file_content = self.context.file_cache.fetch(source_id).unwrap();
        let parsed = match Parser::parse_file(source_id, file_content) {
            Ok(parsed) => parsed,
            Err(e) => {
                message_sender.send(e).expect("Failed to send message");
                return;
            }
        };

        if self.context.emit_irs {
            self.write_parsed_to_file(&parsed);
        }

        let mut source_unit = AstToCAst::transform(&parsed, source_id.filename());
        let header_unit: CTranslationUnit = ExtractHeader::extract(&mut source_unit);

        let source = self.write_c_to_file(&source_unit, &self.context.build_dir);
        c_gen::format_generated(source);
        let header = self.write_c_to_file(&header_unit, &self.context.build_dir);
        c_gen::format_generated(header);

        let main_file = c_gen::build_c_main_file(source_id.filename());
        let main_file = self.write_c_to_file(&main_file, &self.context.build_dir);
        c_gen::format_generated(main_file);

        copy_core_c(&self.context.build_dir);

        let options = RunnerOptions {
            output_name: String::from("out"),
            output_path: self.context.build_dir.clone(),
            build_files: vec![
                self.context.build_dir.join("*.h"),
                self.context.build_dir.join("*.c"),
            ],
            optimization_level: OptLevel::Debug,
        };

        let compiler_runner = Gcc::new();
        if !compiler_runner.is_available() {
            panic!("{:?} is not available", compiler_runner.name());
        }

        let out = match compiler_runner.run(options) {
            Ok(s) => s,
            Err(e) => panic!("Failed C Compilation: {}", e),
        };

        if self.context.run_immediately {
            let start = std::time::Instant::now();
            let _ = std::process::Command::new(out)
                .spawn()
                .unwrap()
                .wait_with_output();
            let end = std::time::Instant::now();
            let duration = end - start;
            println!("Ran in {:?}", duration);
        }
    }

    fn write_parsed_to_file(&self, program: &Program) {
        let dir = self.context.build_dir.join("parsed.txt");
        let mut file = std::fs::File::create(dir).unwrap();
        std::io::Write::write_all(&mut file, format!("{:#?}", program).as_bytes()).unwrap();
    }

    fn write_c_to_file(&self, c: &CTranslationUnit, path: &Path) -> PathBuf {
        let ext = match c.is_header {
            true => "h",
            false => "c",
        };
        let s = CAstWriter::write_unit(c);
        let dir = path.join(&c.name).with_extension(ext);
        let mut file = std::fs::File::create(&dir).unwrap();
        std::io::Write::write_all(&mut file, s.as_bytes()).unwrap();
        dir
    }
}
