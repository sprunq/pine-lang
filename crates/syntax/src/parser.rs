use crate::{
    ast::{expr::*, stmt::*, ProgramUnit},
    lexer::Lexer,
    token::Token,
};
use base::{located::Located, source_id::SourceId};
use messages::{message::Message, parser::ParserError};
use std::{collections::HashMap, hash::Hash, sync::mpsc::Sender};

trait PrefixParselet {
    fn parse(&self, parser: &mut Parser) -> Result<Expr, ParserError>;
}

trait InfixParser {
    fn parse(&self, parser: &mut Parser, left: Expr) -> Result<Expr, ParserError>;
    fn prescedence(&self) -> u8;
}

pub struct Parser<'a> {
    source_id: SourceId,
    message_sender: Sender<Message>,
    lexer: Lexer<'a>,

    current: Located<Token>,
    pending: Option<Located<Token>>,

    prefix_parsers: HashMap<u8, &'a dyn PrefixParselet>,
    infix_parsers: HashMap<u8, &'a dyn InfixParser>,
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
            prefix_parsers: HashMap::new(),
            infix_parsers: HashMap::new(),
        };

        parser.move_next().unwrap();
        parser.move_next().unwrap();

        parser
    }

    fn register_prefix(&mut self, token: Token, parser: &'a dyn PrefixParselet) {
        self.prefix_parsers.insert(token, parser);
    }

    fn register_infix(&mut self, token: Token, parser: &'a dyn InfixParser) {
        self.infix_parsers.insert(token, parser);
    }

    fn move_next(&mut self) -> Result<(), Message> {
        if let Some(token) = self.pending.take() {
            self.current = token;
        }

        match self.lexer.next() {
            Some(Err(e)) => Err(e.into()),
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

    fn expect(&mut self, token: Token) -> Result<(), Message> {
        if self.current.value == token {
            let t = self.current.clone();
            self.move_next()?;
            Ok(())
        } else {
            let t = self.current.clone();
            Err(ParserError::UnrecognizedToken {
                token: t.as_str_loc(),
                expected: token.to_string(),
            }
            .into())
        }
    }

    fn send_message<M: Into<Message>>(&self, message: M) {
        let m = message.into();
        self.message_sender.send(m).unwrap();
    }

    pub fn parse(&mut self) -> Result<ProgramUnit, ()> {
        let mut stmts = vec![];
        while self.pending.is_some() {
            let stmt = self.parse_stmt()?;
            stmts.push(stmt);
        }
        Ok(ProgramUnit::new(stmts))
    }

    fn parse_stmt(&mut self) -> Result<Located<Stmt>, ()> {
        let stmt = match self.current.value {};

        Ok(stmt)
    }

    fn parse_literal_expr(&mut self) -> Result<ExprLiteral, ()> {
        let expr = match self.current.value {
            Token::False => ExprLiteral::Bool(false),
            Token::True => ExprLiteral::Bool(true),
            Token::Integer(i) => ExprLiteral::Integer(i),
            Token::Float(fl) => ExprLiteral::Float(fl),
            Token::String(s) => ExprLiteral::String(s),
            _ => todo!(),
        };

        Ok(expr)
    }

    fn parse_located<F: FnOnce(&mut Self) -> Result<T, ()>, T>(
        &mut self,
        f: F,
    ) -> Result<Located<T>, ()> {
        let start = self.current.span.start;
        let value = f(self)?;
        let end = self.current.span.end;
        let span = start..end;
        let source = self.source_id;
        Ok(Located::new(source, span, value))
    }
}

fn get_op_prescedence(op: &Token) -> Option<u8> {
    match op {
        Token::ParenOpen => Some(0),
        Token::Bang => Some(1),
        Token::Asterisk | Token::Slash | Token::Modulo => Some(2),
        Token::Plus | Token::Minus => Some(3),
        Token::Less | Token::Greater | Token::LessEqual | Token::GreaterEqual => Some(4),
        Token::EqualEqual | Token::BangEqual => Some(5),
        Token::And => Some(6),
        Token::Or => Some(7),
        _ => unreachable!(),
    }
}
