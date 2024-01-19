#![allow(dead_code)]
use crate::{
    ast::{
        expr::Identifier,
        stmt::{Block, TypedParam},
        toplevel::{FunctionDeclaration, TypeObject},
        types::Type,
        Program, TopLevelS,
    },
    token::Token,
};
use base::{located::Spanned, source_id::SourceId};
use messages::{message::Message, parser::ParserError};

#[allow(dead_code)]
struct Precedence {
    level: u8,
}

pub struct Parser<I> {
    tokens: I,
    source_id: SourceId,
    current: Spanned<Token>,
    peek: Spanned<Token>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = Spanned<Token>>,
{
    pub fn new(tokens: I, source_id: SourceId) -> Self {
        let mut p = Self {
            tokens,
            source_id,
            current: Spanned::new(0..0, Token::Plus), // dummy value
            peek: Spanned::new(0..0, Token::Plus),    // dummy value
        };
        p.next(); // read first to peek
        p.next(); // move peek to current and read next
        p
    }

    pub fn parse(tokens: I, source_id: SourceId) -> Result<Program, Message> {
        let mut parser = Parser::new(tokens, source_id);
        parser.parse_program().map_err(|e| e.into())
    }

    fn next(&mut self) {
        if self.peek_v() == Token::Eof {
            self.current = self.peek.clone();
            return;
        }
        let new_peek = self.tokens.next().expect("no more tokens");
        std::mem::swap(&mut self.current, &mut self.peek);
        self.peek = new_peek;
    }

    fn peek_v(&self) -> Token {
        self.peek.value
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParserError> {
        if self.current.value != expected {
            return Err(ParserError::new_unrecognized_token(
                self.source_id,
                &self.current,
                expected,
            ));
        }
        self.next();
        Ok(())
    }

    fn parse_program(&mut self) -> Result<Program, ParserError> {
        let mut stmts = Vec::new();
        while self.peek_v() != Token::Eof {
            stmts.push(self.parse_top_level()?);
        }
        Ok(Program {
            stmts,
            source: self.source_id,
        })
    }

    fn parse_top_level(&mut self) -> Result<TopLevelS, ParserError> {
        todo!()
    }

    /// fun ident(a : Alpha, b : u64) -> u64:
    ///    block
    fn parse_function_declaration(&mut self) -> Result<FunctionDeclaration, ParserError> {
        self.expect(Token::Fun)?;
        let name = self.parse_identifier()?;
        self.expect(Token::LParen)?;
        let params = self.parse_typed_params()?;
        self.expect(Token::RParen)?;
        self.expect(Token::ArrowRight)?;
        let ret_ty = self.parse_type()?;
        self.expect(Token::Colon)?;
        let body = self.parse_block()?;
        Ok(FunctionDeclaration::new(name, params, ret_ty, body))
    }

    fn parse_type_object(&mut self) -> Result<TypeObject, ParserError> {
        todo!()
    }

    fn parse_identifier(&mut self) -> Result<Spanned<Identifier>, ParserError> {
        if let Token::Identifier(ident) = &self.current.value {
            let ident = ident.as_str().into();
            let ident = self.current.with_new_value(ident);
            self.next();
            Ok(ident)
        } else {
            Err(ParserError::new_unrecognized_token(
                self.source_id,
                &self.current,
                "identifier",
            ))
        }
    }

    fn parse_typed_params(&mut self) -> Result<Vec<TypedParam>, ParserError> {
        let mut params = Vec::new();

        let mut parse_one = |this: &mut Self| -> Result<(), ParserError> {
            let name = this.parse_identifier()?;
            this.expect(Token::Colon)?;
            let ty = this.parse_type()?;
            params.push(TypedParam::new(name, ty));
            Ok(())
        };

        // parse first param
        if matches!(self.peek_v(), Token::Identifier(_)) {
            parse_one(self)?;
        }
        loop {
            // all next params are delimited by comma
            if self.peek_v() != Token::Comma {
                break;
            }
            self.next();
            // parse next param
            parse_one(self)?;
        }

        Ok(params)
    }

    fn parse_type(&mut self) -> Result<Spanned<Type>, ParserError> {
        let ty = match self.current.value {
            Token::TyBool => Type::Bool,
            Token::TyI8 => Type::I8,
            Token::TyI32 => Type::I32,
            Token::TyI64 => Type::I64,
            Token::TyU8 => Type::U8,
            Token::TyU32 => Type::U32,
            Token::TyU64 => Type::U64,
            Token::TyF32 => Type::F32,
            Token::TyF64 => Type::F64,
            Token::TyStr => Type::String,
            Token::Identifier(ident) => Type::Struct(ident.as_str().into()),
            _ => {
                return Err(ParserError::new_expected_type(
                    self.source_id,
                    &self.current,
                ))
            }
        };

        Ok(self.current.with_new_value(ty))
    }

    fn parse_block(&mut self) -> Result<Block, ParserError> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexer::{self, Lexer};
    use base::source_id::SourceId;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref SETTINGS: insta::Settings = {
            let mut settings = insta::Settings::new();
            settings.add_redaction(".**.span", "");
            settings
        };
    }

    fn parser(input: &str) -> Parser<lexer::Lexer<'_>> {
        let tokens = Lexer::new(SourceId::from_path(""), input);
        Parser::new(tokens, "".into())
    }

    #[test]
    fn test_parse_identifier() {
        let input = r#"hello_world"#;
        let parsed = parser(input).parse_identifier();
        SETTINGS.bind(|| {
            insta::assert_json_snapshot!(parsed);
        });
    }

    #[test]
    fn test_parse_identifier_fail() {
        let input = r#"1.0 + hello_world"#;
        let parsed = parser(input).parse_identifier();
        SETTINGS.bind(|| {
            insta::assert_json_snapshot!(parsed);
        });
    }

    #[test]
    fn test_parse_type() {
        let input = r#"bool"#;
        let parsed = parser(input).parse_type();
        SETTINGS.bind(|| {
            insta::assert_json_snapshot!(parsed);
        });
    }

    #[test]
    fn test_parse_type_fail() {
        let input = r#"1.0 + bool"#;
        let parsed = parser(input).parse_type();
        SETTINGS.bind(|| {
            insta::assert_json_snapshot!(parsed);
        });
    }

    #[test]
    fn test_parse_typed_params() {
        let input = r#"a : Alpha, b : u64"#;
        let parsed = parser(input).parse_typed_params();
        SETTINGS.bind(|| {
            insta::assert_json_snapshot!(parsed);
        });
    }
}
