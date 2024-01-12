use crate::{ast::Program, lexer::Lexer, token::Token};
use base::{located::Located, source_id::SourceId};
use messages::{lexer::LexerError, message::Message, parser::ParserError};

pub struct Parser {}

impl Parser {
    pub fn parse_file(source: SourceId, file: &str) -> Result<Program, Message> {
        let _lexer = Lexer::new(source, file);
        todo!()
        //     let parser = Parser::new();
        //     let parse_res = parser.parse(source, lexer);

        //     parse_res.map_err(|e| Self::uplift_parse_err(source, file, e))
    }
}
