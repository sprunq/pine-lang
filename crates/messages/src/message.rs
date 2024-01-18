use crate::{lexer::LexerError, parser::ParserError};
use base::source_id::SourceId;
use codespan_reporting::diagnostic::Diagnostic;

pub enum Message {
    Lexer(LexerError),
    Parse(ParserError),
}

impl Message {
    pub fn as_diagnostic(&self) -> Diagnostic<SourceId> {
        match self {
            Message::Lexer(err) => err.as_diagnostic(),
            Message::Parse(err) => err.as_diagnostic(),
        }
    }

    pub fn origin(&self) -> SourceId {
        match self {
            Message::Lexer(err) => err.origin(),
            Message::Parse(err) => err.origin(),
        }
    }
}

impl From<LexerError> for Message {
    fn from(err: LexerError) -> Self {
        Message::Lexer(err)
    }
}

impl From<ParserError> for Message {
    fn from(err: ParserError) -> Self {
        Message::Parse(err)
    }
}
