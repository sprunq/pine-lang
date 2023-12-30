use base::{located::Located, source_id::SourceId};
use codespan_reporting::diagnostic::{Diagnostic, Label};

#[derive(Debug, PartialEq, Clone)]
pub enum ParserError {
    ExtraToken {
        token: Located<String>,
    },
    InvalidToken {
        location: Located<()>,
    },
    UnrecognizedEOF {
        location: Located<()>,
        expected: Vec<String>,
    },
    UnrecognizedToken {
        token: Located<String>,
        expected: String,
    },
}

impl ParserError {
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
            ParserError::ExtraToken { .. } => 0,
            ParserError::InvalidToken { .. } => 1,
            ParserError::UnrecognizedEOF { .. } => 2,
            ParserError::UnrecognizedToken { .. } => 3,
        }
    }

    pub fn message(&self) -> &str {
        match self {
            ParserError::ExtraToken { .. } => "extra token",
            ParserError::InvalidToken { .. } => "invalid token",
            ParserError::UnrecognizedEOF { .. } => "unrecognized EOF",
            ParserError::UnrecognizedToken { .. } => "unrecognized token",
        }
    }

    pub fn labels(&self) -> Vec<Label<SourceId>> {
        match self {
            ParserError::ExtraToken { token } => {
                vec![Label::primary(token.source, token.span.clone())]
            }
            ParserError::InvalidToken { location } => {
                vec![Label::primary(location.source, location.span.clone())]
            }
            ParserError::UnrecognizedEOF {
                location,
                expected: _,
            } => {
                vec![Label::primary(location.source, location.span.clone())]
            }
            ParserError::UnrecognizedToken { token, expected: _ } => {
                vec![Label::primary(token.source, token.span.clone())]
            }
        }
    }

    pub fn notes(&self) -> Vec<String> {
        match self {
            ParserError::ExtraToken { .. } => vec![],
            ParserError::InvalidToken { .. } => vec![],
            ParserError::UnrecognizedEOF {
                location: _,
                expected,
            } => {
                vec![format!("expected: {}", one_of(expected))]
            }
            ParserError::UnrecognizedToken { token: _, expected } => {
                vec![format!("expected: {}", expected)]
            }
        }
    }
}

fn one_of(tokens: &[String]) -> String {
    let (token_last, tokens) = match tokens.split_last() {
        Some((token_last, &[])) => return token_last.to_string(),
        Some((token_last, tokens)) => (token_last, tokens),
        None => return "nothing".to_string(),
    };

    let mut output = String::new();
    for token in tokens {
        output.push_str(token);
        output.push_str(", ");
    }
    output.push_str("or ");
    output.push_str(token_last);
    output
}
