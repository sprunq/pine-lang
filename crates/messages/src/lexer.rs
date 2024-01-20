use base::{located::Located, source_id::SourceId};
use codespan_reporting::diagnostic::{Diagnostic, Label};

#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub enum LexerError {
    UnexpectedCharacter { token: Located<char> },
    IndentationNotMultipleFour { token: Located<()>, found: usize },
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
            LexerError::UnexpectedCharacter { .. } => 0,
            LexerError::IndentationNotMultipleFour { .. } => 1,
        }
    }

    pub fn message(&self) -> &str {
        match self {
            LexerError::UnexpectedCharacter { .. } => "unexpected input",
            LexerError::IndentationNotMultipleFour { .. } => "invalid indentation",
        }
    }

    pub fn labels(&self) -> Vec<codespan_reporting::diagnostic::Label<SourceId>> {
        match self {
            LexerError::UnexpectedCharacter { token } => {
                vec![Label::primary(token.source, token.located.span.clone())]
            }
            LexerError::IndentationNotMultipleFour { token, .. } => {
                vec![Label::primary(token.source, token.located.span.clone())]
            }
        }
    }

    pub fn notes(&self) -> Vec<String> {
        match self {
            LexerError::UnexpectedCharacter { .. } => {
                vec![]
            }
            LexerError::IndentationNotMultipleFour { found, .. } => {
                vec![format!(
                    "indentation must be a multiple of 4, but found {}",
                    found
                )]
            }
        }
    }

    pub fn origin(&self) -> SourceId {
        match self {
            LexerError::UnexpectedCharacter { token } => token.source,
            LexerError::IndentationNotMultipleFour { token, .. } => token.source,
        }
    }
}
