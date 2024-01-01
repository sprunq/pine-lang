use crate::{ast::Program, grammar, lexer::Lexer, token::Token};
use base::{located::Located, source_id::SourceId};
use lalrpop_util::ParseError as LalrpopParseError;
use messages::{lexer::LexerError, message::Message, parser::ParserError};

pub type Parser = grammar::ProgramParser;

impl Parser {
    pub fn parse_file(source: SourceId, file: &str) -> Result<Program, Message> {
        let lexer = Lexer::new(source, file);
        let parser = Parser::new();
        let parse_res = parser.parse(source, lexer);

        parse_res.map_err(|e| Self::uplift_parse_err(source, file, e))
    }

    /// Converts a lalrpop `ParseError` into our internal representation of an error.
    pub fn uplift_parse_err(
        source: SourceId,
        file: &str,
        err: LalrpopParseError<usize, Token, LexerError>,
    ) -> Message {
        match err {
            LalrpopParseError::ExtraToken {
                token: (start, _, end),
            } => ParserError::ExtraToken {
                token: Located::new(source, start..end, file[start..end].to_string()),
            }
            .into(),
            LalrpopParseError::InvalidToken { location } => ParserError::InvalidToken {
                location: Located::empty(source, location..location + 1),
            }
            .into(),
            LalrpopParseError::UnrecognizedEOF { location, expected } => {
                ParserError::UnrecognizedEOF {
                    location: Located::empty(source, location..location + 1),
                    expected,
                }
                .into()
            }
            LalrpopParseError::UnrecognizedToken {
                token: (start, _, end),
                expected,
            } => ParserError::UnrecognizedToken {
                token: Located::new(source, start..end, file[start..end].to_string()),
                expected,
            }
            .into(),
            LalrpopParseError::User { error } => error.into(),
        }
    }
}
