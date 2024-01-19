use base::{compile_context::CompileContext, source_id::SourceId};
use messages::message::Message;
use std::sync::mpsc::Sender;
use syntax::{ast::Program, lexer::Lexer, parser::Parser};

pub struct Compiler<'a> {
    context: &'a mut CompileContext,
}

impl<'a> Compiler<'a> {
    pub fn new(context: &'a mut CompileContext) -> Self {
        Self { context }
    }

    pub fn compile(&mut self, msg_sender: Sender<Message>) {
        let source_id = SourceId::from_path(&self.context.build_pkg);

        let file_content = self.context.file_cache.fetch(source_id).unwrap();

        let lexer = Lexer::new(source_id, &file_content);

        let parsed = match Parser::parse(lexer) {
            Ok(parsed) => parsed,
            Err(e) => {
                msg_sender.send(e).expect("Failed to send message");
                return;
            }
        };

        if self.context.emit_irs {
            self.write_parsed_to_file(&parsed);
        }
    }

    fn write_parsed_to_file(&self, program: &Program) {
        let dir = self.context.build_dir.join("parsed.txt");
        let mut file = std::fs::File::create(dir).unwrap();
        std::io::Write::write_all(&mut file, format!("{:#?}", program).as_bytes()).unwrap();
    }
}
