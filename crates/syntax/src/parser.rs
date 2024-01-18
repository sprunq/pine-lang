use crate::{ast::Program, lexer::Lexer, token::Token};
use base::{located::Located, source_id::SourceId};
use messages::{lexer::LexerError, message::Message, parser::ParserError};

struct Precedence {
    level: u8,
}

pub struct Parser<I> {
    tokens: I,
    current: Located<Token>,
    peek: Located<Token>,
}

impl<I> Parser<I>
where
    I: Iterator<Item = Located<Token>>,
{
    pub fn new(tokens: I) -> Self {
        let mut p = Self {
            tokens,
            current: Located::new("/".into(), 0..0, Token::Eof),
            peek: Located::new("/".into(), 0..0, Token::Eof),
        };
        p.next();
        p.next();
        p
    }

    pub fn parse(tokens: I) -> Result<Program, Message> {
        let parser = Parser::new(tokens);
        todo!()
    }

    fn next(&mut self) {
        let new_peek = self.tokens.next().expect("no more tokens");
        std::mem::swap(&mut self.current, &mut self.peek);
        self.peek = new_peek;
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParserError> {
        if self.peek.value != expected {
            return Err(ParserError::new_unrecognized_token(&self.peek, expected));
        }
        Ok(())
    }
}
