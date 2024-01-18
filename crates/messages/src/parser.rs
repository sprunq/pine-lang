use base::{located::Located, source_id::SourceId};
use codespan_reporting::diagnostic::{Diagnostic, Label};

#[derive(Debug, PartialEq, Clone)]
pub enum ParserError {
    UnexpectedEof {
        location: Located<()>,
    },
    UnrecognizedToken {
        token: Located<String>,
        expected: String,
    },
}

impl ParserError {
    pub fn new_unrecognized_token<F, E>(found: &Located<F>, expected: E) -> ParserError
    where
        F: ToString,
        E: ToString,
    {
        let token = found.map_value(|t| t.to_string());
        let expected = expected.to_string();
        ParserError::UnrecognizedToken { token, expected }
    }

    pub fn as_diagnostic(&self) -> Diagnostic<SourceId> {
        let code = self.code();
        let message = self.message();
        let labels = self.labels();
        let notes = self.notes();
        Diagnostic::error()
            .with_code(format!("SYN::{:04}", code).as_str())
            .with_message(message)
            .with_labels(labels)
            .with_notes(notes)
    }

    pub fn code(&self) -> usize {
        match self {
            ParserError::UnexpectedEof { .. } => 1,
            ParserError::UnrecognizedToken { .. } => 2,
        }
    }

    pub fn message(&self) -> &str {
        match self {
            ParserError::UnexpectedEof { .. } => "unrecognized EOF",
            ParserError::UnrecognizedToken { .. } => "unrecognized token",
        }
    }

    pub fn labels(&self) -> Vec<Label<SourceId>> {
        match self {
            ParserError::UnexpectedEof { location } => {
                vec![Label::primary(location.source, location.span.clone())]
            }
            ParserError::UnrecognizedToken { token, expected: _ } => {
                vec![Label::primary(token.source, token.span.clone())]
            }
        }
    }

    pub fn notes(&self) -> Vec<String> {
        match self {
            ParserError::UnexpectedEof { location: _ } => {
                vec![]
            }
            ParserError::UnrecognizedToken { token: _, expected } => {
                vec![format!("expected: {}", expected)]
            }
        }
    }

    pub fn origin(&self) -> SourceId {
        match self {
            ParserError::UnexpectedEof { location } => location.source,
            ParserError::UnrecognizedToken { token, expected: _ } => token.source,
        }
    }
}
