use crate::{ast::Program, lexer::Lexer, token::Token};
use base::{located::Located, source_id::SourceId};
use messages::{message::Message, parser::ParserError};
use std::sync::mpsc::Sender;

pub struct Parser<'a> {
    source_id: SourceId,
    message_sender: Sender<Message>,
    lexer: Lexer<'a>,

    current: Located<Token>,
    pending: Option<Located<Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(source_id: SourceId, content: &'a str, message_sender: Sender<Message>) -> Self {
        let lexer = Lexer::new(source_id, content);
        let p0 = Located::new(source_id, 0..0, Token::Error);

        let mut parser = Self {
            source_id,
            message_sender,
            lexer,
            current: p0,
            pending: None,
        };

        parser.next().expect("hm");
        parser.next().expect("hmm");

        parser
    }

    fn next(&mut self) -> Result<(), ()> {
        if let Some(token) = self.pending.take() {
            self.current = token;
        }

        match self.lexer.next() {
            Some(Err(e)) => {
                self.send_message(e);
                Err(())
            }
            Some(Ok(token)) => {
                self.pending = Some(token);
                Ok(())
            }
            None => {
                self.pending = None;
                Ok(())
            }
        }
    }

    fn expect(&mut self, token: Token) -> Result<Located<Token>, ()> {
        if self.current.value == token {
            let t = self.current.clone();
            self.next()?;
            Ok(t)
        } else {
            let t = self.current.clone();
            self.send_message(ParserError::UnrecognizedToken {
                token: t.as_str_loc(),
                expected: token.to_string(),
            });
            Err(())
        }
    }

    fn send_message<M: Into<Message>>(&self, message: M) {
        let m = message.into();
        self.message_sender.send(m).unwrap();
    }

    pub fn parse(&mut self) -> Result<Program, ()> {
        todo!()
    }
}
