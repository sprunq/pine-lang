use base::{located::Located, source_id::SourceId};
use codespan_reporting::diagnostic::{Diagnostic, Label};

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub enum ParserError {
    UnexpectedEof {
        location: Located<()>,
    },
    UnrecognizedToken {
        found: Located<String>,
        expected: String,
    },
    ExpectedType {
        found: Located<String>,
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
        ParserError::UnrecognizedToken {
            found: token,
            expected,
        }
    }

    pub fn new_expected_type<F>(found: &Located<F>) -> ParserError
    where
        F: ToString,
    {
        let token = found.map_value(|t| t.to_string());
        ParserError::ExpectedType { found: token }
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
            ParserError::ExpectedType { .. } => 3,
        }
    }

    pub fn message(&self) -> &str {
        match self {
            ParserError::UnexpectedEof { .. } => "unrecognized EOF",
            ParserError::UnrecognizedToken { .. } => "unrecognized token",
            ParserError::ExpectedType { .. } => "expected type",
        }
    }

    pub fn labels(&self) -> Vec<Label<SourceId>> {
        match self {
            ParserError::UnexpectedEof { location } => {
                vec![Label::primary(location.source, location.span.clone())]
            }
            ParserError::UnrecognizedToken {
                found: token,
                expected: _,
            } => {
                vec![Label::primary(token.source, token.span.clone())]
            }
            ParserError::ExpectedType { found } => {
                vec![Label::primary(found.source, found.span.clone())]
            }
        }
    }

    pub fn notes(&self) -> Vec<String> {
        match self {
            ParserError::UnexpectedEof { location: _ } => {
                vec![]
            }
            ParserError::UnrecognizedToken { found: _, expected } => {
                vec![format!("expected: {}", expected)]
            }
            ParserError::ExpectedType { found } => {
                vec![format!("found: {}", found.value)]
            }
        }
    }

    pub fn origin(&self) -> SourceId {
        match self {
            ParserError::UnexpectedEof { location } => location.source,
            ParserError::UnrecognizedToken {
                found: token,
                expected: _,
            } => token.source,
            ParserError::ExpectedType { found } => found.source,
        }
    }
}
