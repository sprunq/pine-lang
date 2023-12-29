use crate::token::Token;
use base::{located::Located, source_id::SourceId};
use logos::Logos;
use messages::lexer::LexerError;

#[derive(Debug)]
pub struct Lexer<'a> {
    source_id: SourceId,
    inner: logos::Lexer<'a, Token>,
    pending: Option<(usize, Token, usize)>,
}

impl<'a> Lexer<'a> {
    pub fn new(source_id: SourceId, source: &'a str) -> Self {
        Self {
            source_id,
            inner: Token::lexer(source),
            pending: None,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<(usize, Token, usize), LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.pending.take() {
            return Some(Ok(token));
        }

        match self.inner.next()? {
            Token::Error => {
                let mut span = self.inner.span();

                // Check for unterminated string.
                if self.inner.slice().starts_with('"') {
                    return Some(Err(LexerError::UnterminatedString {
                        location: Located::empty(self.source_id, span),
                    }));
                }

                // Recover error.
                while let Some(token) = self.inner.next() {
                    let span_new = self.inner.span();
                    if span.end == span_new.start {
                        span.end = span_new.end;
                    } else {
                        self.pending = Some((span_new.start, token, span_new.end));
                        break;
                    }
                }

                Some(Err(LexerError::UnexpectedInput {
                    token: Located::new(
                        self.source_id,
                        span.clone(),
                        self.inner.source()[span.start..span.end].to_string(),
                    ),
                }))
            }
            token => {
                let span = self.inner.span();
                Some(Ok((span.start, token, span.end)))
            }
        }
    }
}
