#![allow(dead_code)]
use crate::{
    ast::{
        expr::Identifier,
        stmt::{Block, Stmt, TypedParam},
        toplevel::{FunctionDeclaration, TypeObject},
        types::Type,
        Program, TopLevelS,
    },
    token::Token,
};
use base::{source_id::SourceId, spanned::Spanned};
use messages::{lexer::LexerError, message::Message, parser::ParserError};

#[allow(dead_code)]
struct Precedence {
    level: u8,
}

macro_rules! spanned {
    ($self:ident, $body:block) => {{
        let s = $self.current.span.start;
        let value = $body;
        let e = $self.current.span.end;
        let spanned = Spanned::new(s..e, value);
        Ok(spanned)
    }};
}

pub struct Parser<I> {
    tokens: I,
    source_id: SourceId,
    current: Spanned<Token>,
    peek: Spanned<Token>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = Result<Spanned<Token>, LexerError>>,
{
    pub fn new(tokens: I, source_id: SourceId) -> Self {
        let mut p = Self {
            tokens,
            source_id,
            current: Spanned::new(0..0, Token::Plus), // dummy value
            peek: Spanned::new(0..0, Token::Plus),    // dummy value
        };
        p.next().unwrap(); // read first to peek
        p.next().unwrap(); // move peek to current and read next
        p
    }

    pub fn parse(tokens: I, source_id: SourceId) -> Result<Program, Message> {
        let mut parser = Parser::new(tokens, source_id);
        parser.parse_program()
    }

    fn next(&mut self) -> Result<(), Message> {
        if self.peek_v() == Token::Eof {
            self.current = self.peek.clone();
            return Ok(());
        }
        let new_peek = self.tokens.next();
        match new_peek {
            Some(new_peek) => match new_peek {
                Ok(new_peek) => {
                    std::mem::swap(&mut self.current, &mut self.peek);
                    self.peek = new_peek;
                    Ok(())
                }
                Err(e) => Err(e.into()),
            },
            None => Err(ParserError::new_unexpected_eof(
                self.source_id,
                self.peek.clone().map_value(|_| ()),
            )
            .into()),
        }
    }

    fn peek_v(&self) -> Token {
        self.peek.value
    }

    fn current_v(&self) -> Token {
        self.current.value
    }

    fn expect(&mut self, expected: Token) -> Result<(), Message> {
        if self.current.value != expected {
            return Err(ParserError::new_unrecognized_token(
                self.source_id,
                self.current.clone(),
                expected,
            )
            .into());
        }
        self.next().unwrap();
        Ok(())
    }

    fn parse_program(&mut self) -> Result<Program, Message> {
        let mut stmts = Vec::new();
        while self.peek_v() != Token::Eof {
            stmts.push(self.parse_top_level()?);
        }
        Ok(Program {
            stmts,
            source: self.source_id,
        })
    }

    fn parse_top_level(&mut self) -> Result<TopLevelS, Message> {
        match self.current_v() {
            Token::Fun => self.parse_func_decl().map(|e| e.map_value(|e| e.into())),
            Token::Type => self.parse_type_object().map(|e| e.map_value(|e| e.into())),
            _ => Err(ParserError::new_unrecognized_token(
                self.source_id,
                self.current.clone(),
                "top level declaration",
            )
            .into()),
        }
    }

    /// fun ident(a : Alpha, b : u64) -> u64:
    ///    block
    fn parse_func_decl(&mut self) -> Result<Spanned<FunctionDeclaration>, Message> {
        spanned!(self, {
            self.expect(Token::Fun)?;
            let name = self.parse_identifier()?;
            self.expect(Token::LParen)?;
            let params = self.parse_typed_params()?;
            self.expect(Token::RParen)?;
            self.expect(Token::ArrowRight)?;
            let ret_ty = self.parse_type()?;
            self.expect(Token::Colon)?;
            let body = self.parse_block()?.value;
            FunctionDeclaration::new(name, params, ret_ty, body)
        })
    }

    fn parse_type_object(&mut self) -> Result<Spanned<TypeObject>, Message> {
        todo!()
    }

    fn parse_identifier(&mut self) -> Result<Spanned<Identifier>, Message> {
        if let Token::Identifier(ident) = &self.current.value {
            let ident = ident.as_str().into();
            let ident = self.current.from_new_value(ident);
            self.next()?;
            Ok(ident)
        } else {
            Err(ParserError::new_unrecognized_token(
                self.source_id,
                self.current.clone(),
                "identifier",
            )
            .into())
        }
    }

    /// [ ident : type, ident : type, ... ]
    fn parse_typed_params(&mut self) -> Result<Vec<TypedParam>, Message> {
        let mut params = Vec::new();

        // indent : type
        let mut parse_one = |this: &mut Self| -> Result<(), Message> {
            let name = this.parse_identifier()?;
            this.expect(Token::Colon)?;
            let ty = this.parse_type()?;
            params.push(TypedParam::new(name, ty));
            Ok(())
        };

        // parse first param
        if matches!(self.current_v(), Token::Identifier(_)) {
            parse_one(self)?;
        }

        // all next params are delimited by comma
        while self.current_v() == Token::Comma {
            self.next()?;
            // parse next param
            parse_one(self)?;
        }

        Ok(params)
    }

    fn parse_type(&mut self) -> Result<Spanned<Type>, Message> {
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
            Token::Underscore => Type::Unit,
            Token::Identifier(ident) => Type::Struct(ident.as_str().into()),
            _ => {
                return Err(
                    ParserError::new_expected_type(self.source_id, self.current.clone()).into(),
                )
            }
        };
        self.next()?;
        Ok(self.current.from_new_value(ty))
    }

    fn parse_block(&mut self) -> Result<Spanned<Block>, Message> {
        todo!("Not finished");

        let mut stmts = Vec::new();

        let s = self.current.span.start;
        while self.current_v() == Token::NewLine {
            stmts.push(self.parse_stmt()?);
        }
        let block = Block::new(stmts);

        let e = self.current.span.end;
        let spanned = Spanned::new(s..e, block);
        Ok(spanned)
    }

    fn parse_stmt(&mut self) -> Result<Spanned<Stmt>, Message> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexer::Lexer;
    use base::source_id::SourceId;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref SETTINGS: insta::Settings = {
            let mut settings = insta::Settings::new();
            settings.set_snapshot_path("test_outputs/parser");
            settings.add_redaction(".**.span", "");
            settings
        };
    }

    /// Assert snapshot with redactions
    macro_rules! insta_assert {
        ($parsed:expr) => {{
            SETTINGS.bind(|| {
                insta::assert_ron_snapshot!($parsed);
            });
        }};
    }

    /// Read test file and parse it
    macro_rules! from_pine {
        ($name:expr) => {{
            parser(&read_test_file($name))
        }};
    }

    fn read_test_file(s: &str) -> String {
        let current_dir = std::env::current_dir().unwrap();
        let path = current_dir
            .join("src")
            .join("test_inputs")
            .join("parser")
            .join(s)
            .with_extension("pine");
        let read_result = std::fs::read_to_string(&path);
        assert!(read_result.is_ok(), "failed to read file: {:?}", path);
        read_result.unwrap()
    }

    fn parser(input: &str) -> Parser<Lexer<'_>> {
        let tokens = Lexer::new(SourceId::from_path(""), input);
        Parser::new(tokens, "".into())
    }

    #[test]
    fn test_parse_identifier() {
        let parsed = from_pine!("identifier").parse_identifier();
        insta_assert!(parsed);
    }

    #[test]
    fn test_parse_identifier_fail() {
        let parsed = from_pine!("identifier_fail").parse_identifier();
        insta_assert!(parsed);
    }

    #[test]
    fn test_parse_type() {
        let parsed = from_pine!("type").parse_type();
        insta_assert!(parsed);
    }

    #[test]
    fn test_parse_type_fail() {
        let parsed = from_pine!("type_fail").parse_type();
        assert!(parsed.is_err());
        insta_assert!(parsed);
    }

    #[test]
    fn test_parse_typed_param_multi() {
        let parsed = from_pine!("param_multi").parse_typed_params();
        insta_assert!(parsed);
    }

    #[test]
    fn test_parse_typed_param_single() {
        let parsed = from_pine!("param_single").parse_typed_params();
        insta_assert!(parsed);
    }

    #[test]
    fn test_parse_typed_param_fail() {
        let parsed = from_pine!("param_fail").parse_typed_params();
        assert!(parsed.is_err());
        insta_assert!(parsed);
    }
}
