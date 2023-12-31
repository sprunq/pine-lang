use base::{located::Located, source_id::SourceId};
use codespan_reporting::diagnostic::{Diagnostic, Label};

#[derive(Debug, PartialEq, Clone)]
pub enum LexerError {
    UnexpectedInput { token: Located<String> },
    UnterminatedString { location: Located<()> },
}

impl LexerError {
    pub fn as_diagnostic(&self) -> Diagnostic<SourceId> {
        let code = self.code();
        let message = self.message();
        let labels = self.labels();
        let notes = self.notes();
        Diagnostic::error()
            .with_code(format!("LEX::{:04}", code).as_str())
            .with_message(message)
            .with_labels(labels)
            .with_notes(notes)
    }

    pub fn code(&self) -> usize {
        match self {
            LexerError::UnexpectedInput { .. } => 0,
            LexerError::UnterminatedString { .. } => 1,
        }
    }

    pub fn message(&self) -> &str {
        match self {
            LexerError::UnexpectedInput { .. } => "unexpected input",
            LexerError::UnterminatedString { .. } => "unterminated string",
        }
    }

    pub fn labels(&self) -> Vec<codespan_reporting::diagnostic::Label<SourceId>> {
        match self {
            LexerError::UnexpectedInput { token } => {
                vec![Label::primary(token.source, token.span.clone())]
            }
            LexerError::UnterminatedString { location } => {
                vec![Label::primary(location.source, location.span.clone())]
            }
        }
    }

    pub fn notes(&self) -> Vec<String> {
        vec![]
    }
}
