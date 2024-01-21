use crate::token::Token;
use base::{located::Located, source_id::SourceId, spanned::Spanned};
use internment::Intern;
use messages::lexer::LexerError;
use std::{cmp::Ordering, collections::VecDeque, str::CharIndices};

#[derive(Debug, Clone)]
pub struct Lexer<'source> {
    pub input: &'source str,
    pub chars: CharIndices<'source>,
    pub source: SourceId,
    prev_line_indent: usize,
    buffer: VecDeque<Spanned<Token>>,
    reached_eof: bool,
    current: (usize, char),
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Result<Spanned<Token>, LexerError>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.reached_eof {
            return None;
        }

        let next = self.next_token();

        if let Ok(t) = &next {
            if t.value == Token::Eof && !self.reached_eof {
                self.reached_eof = true;
            }
        }

        Some(next)
    }
}

impl<'source> Lexer<'source> {
    pub fn new(source: SourceId, input: &'source str) -> Self {
        let mut buffer = VecDeque::new();
        buffer.reserve(64);
        let mut lexer = Self {
            chars: input.char_indices(),
            buffer,
            prev_line_indent: 0,
            reached_eof: false,
            current: (0, '\0'),
            source,
            input,
        };
        lexer.advance(); // initialize current
        lexer
    }

    fn next_token(&mut self) -> Result<Spanned<Token>, LexerError> {
        if let Some(t) = self.buffer.pop_front() {
            return Ok(t);
        }

        self.skip_whitespace_until_nl();

        let start_pos = self.char_pos();

        let tok = match self.char() {
            '\0' => Token::Eof,
            '\r' => {
                self.advance();
                return self.next_token();
            }
            '\n' => {
                self.advance();
                let line_start = self.char_pos();
                self.skip_whitespace_until_nl();
                let indent = self.char_pos() - line_start;

                if indent % 4 != 0 {
                    return Err(LexerError::IndentationNotMultipleFour {
                        token: Located::new(
                            self.source,
                            Spanned::empty(line_start..self.char_end_pos()),
                        ),
                        found: indent,
                    });
                }

                let prev_indent_steps = self.prev_line_indent / 4;
                let indent_steps = indent / 4;
                let steps_diff = prev_indent_steps.abs_diff(indent_steps);

                // we can only return one token at a time, so we need to buffer the extra indents
                let mut push_indents_to_buffer = |token: Token| {
                    for i in 0..steps_diff {
                        let t = Spanned::new(line_start + 4 * i..line_start + 4 * i + 4, token);
                        self.buffer.push_back(t);
                    }
                };

                match indent_steps.cmp(&prev_indent_steps) {
                    Ordering::Less => {
                        self.prev_line_indent = indent;
                        push_indents_to_buffer(Token::Dedent);
                        Token::NewLine
                    }
                    Ordering::Equal => Token::NewLine,
                    Ordering::Greater => {
                        self.prev_line_indent = indent;
                        push_indents_to_buffer(Token::Indent);
                        Token::NewLine
                    }
                }
            }
            '_' => {
                self.advance();
                Token::Underscore
            }
            ':' => {
                self.advance();
                Token::Colon
            }
            ',' => {
                self.advance();
                Token::Comma
            }
            '(' => {
                self.advance();
                Token::LParen
            }
            ')' => {
                self.advance();
                Token::RParen
            }
            '{' => {
                self.advance();
                Token::LBrace
            }
            '}' => {
                self.advance();
                Token::RBrace
            }
            '[' => {
                self.advance();
                Token::LBracket
            }
            ']' => {
                self.advance();
                Token::RBracket
            }
            '=' => {
                self.advance();
                if self.advance_if('=') {
                    Token::EqualEqual
                } else {
                    Token::Equal
                }
            }
            '!' => {
                self.advance();
                if self.advance_if('=') {
                    Token::BangEqual
                } else {
                    Token::Bang
                }
            }
            '>' => {
                self.advance();
                if self.advance_if('=') {
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            }
            '<' => {
                self.advance();
                if self.advance_if('=') {
                    Token::LessEqual
                } else {
                    Token::Less
                }
            }
            '+' => {
                self.advance();
                Token::Plus
            }
            '-' => {
                self.advance();
                if self.advance_if('>') {
                    Token::ArrowRight
                } else {
                    Token::Minus
                }
            }
            '/' => {
                self.advance();
                // line comment
                if self.advance_if('/') {
                    while !matches!(self.char(), '\n' | '\0') {
                        self.advance();
                    }
                    return self.next_token();
                } else {
                    Token::Slash
                }
            }
            '*' => {
                self.advance();
                Token::Asterisk
            }
            '%' => {
                self.advance();
                Token::Modulo
            }
            '.' => {
                self.advance();
                Token::Dot
            }
            '|' => {
                self.advance();
                Token::Pipe
            }
            '"' => {
                let s = self.read_string();
                Token::String(Intern::new(String::from(s)))
            }
            _ if Self::is_letter(self.char()) => {
                let s = self.read_identifier();
                Token::lookup_keyword(s).unwrap_or(Token::Identifier(Intern::new(String::from(s))))
            }
            _ if Self::is_number(self.char()) => {
                let number = self.read_number();
                match number {
                    LexedNumber::Int(i) => Token::Integer(i),
                    LexedNumber::Float(f) => Token::Float(f),
                }
            }
            _ => {
                let span = self.char_pos()..self.char_end_pos();
                let error = LexerError::UnexpectedCharacter {
                    token: Located::new(self.source, Spanned::new(span, self.char())),
                };
                self.advance();
                return Err(error);
            }
        };
        let end_pos = self.char_pos();
        Ok(Spanned::new(start_pos..end_pos, tok))
    }

    #[inline]
    fn read_string(&mut self) -> &str {
        self.advance();
        let start = self.char_pos();
        loop {
            match self.char() {
                '"' => break,
                '\0' | '\n' => panic!("unterminated string"),
                '\\' => {
                    todo!("escape sequences")
                }
                _ => {}
            }
            self.advance();
        }
        let end = self.char_pos();
        self.advance();
        &self.input[start..end]
    }

    #[inline]
    fn read_identifier(&mut self) -> &str {
        let start = self.char_pos();
        while Self::is_letter(self.char()) || self.char().is_ascii_digit() {
            self.advance();
        }
        let end = self.char_pos();
        &self.input[start..end]
    }

    #[inline]
    fn read_number(&mut self) -> LexedNumber {
        let start = self.char_pos();
        while self.char().is_ascii_digit() || self.char() == '.' {
            self.advance();
        }
        let end = self.char_pos();

        let number = &self.input[start..end];
        if number.contains('.') {
            let number = number.parse::<f64>().unwrap();
            LexedNumber::Float(number)
        } else {
            let number = number.parse::<u64>().unwrap();
            LexedNumber::Int(number)
        }
    }

    #[inline]
    fn is_letter(character: char) -> bool {
        character.is_alphabetic() || character == '_'
    }

    #[inline]
    fn is_number(character: char) -> bool {
        character.is_ascii_digit()
    }

    #[inline]
    fn skip_whitespace_until_nl(&mut self) {
        while self.char() != '\n' && self.char() != '\r' && self.char().is_whitespace() {
            self.advance()
        }
    }

    #[inline]
    fn advance(&mut self) {
        self.current = self.chars.next().unwrap_or((self.input.len(), '\0'));
    }

    #[inline]
    fn char(&self) -> char {
        self.current.1
    }

    #[inline]
    fn char_pos(&self) -> usize {
        self.current.0
    }

    #[inline]
    fn char_end_pos(&self) -> usize {
        self.current.0 + self.current.1.len_utf8()
    }

    #[inline]
    fn advance_if(&mut self, ch: char) -> bool {
        if self.char() == ch {
            self.advance();
            true
        } else {
            false
        }
    }
}

enum LexedNumber {
    Int(u64),
    Float(f64),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_token(input: &str, expected: Token) {
        let lexer = Lexer::new(SourceId::from_path(""), input);
        let tokens = lexer.collect::<Vec<_>>();
        let token = tokens[0].clone();
        assert!(token.is_ok());
        assert_eq!(tokens[0].clone().unwrap().value, expected);
        assert_eq!(tokens.len(), 1 + 1); // +1 for EOF
    }

    fn assert_tokens(input: &str, expected: Vec<Token>) {
        let lexer = Lexer::new(SourceId::from_path(""), input);
        let tokens = lexer.collect::<Vec<_>>();
        for (expected, token) in expected.iter().zip(&tokens) {
            assert!(token.is_ok());
            assert_eq!(token.clone().unwrap().value, *expected);
        }
        assert_eq!(tokens.len(), expected.len() + 1); // +1 for EOF
    }

    #[test]
    fn test_indentation_no_indent() {
        assert_tokens(
            r#"
hello
world
"#,
            vec![
                Token::NewLine,
                Token::Identifier("hello".to_string().into()),
                Token::NewLine,
                Token::Identifier("world".to_string().into()),
                Token::NewLine,
            ],
        )
    }

    #[test]
    fn test_indentation_single_up() {
        assert_tokens(
            r#"
hello
    world
    yo
    man 
"#,
            vec![
                Token::NewLine,
                Token::Identifier("hello".to_string().into()),
                Token::NewLine,
                Token::Indent,
                Token::Identifier("world".to_string().into()),
                Token::NewLine,
                Token::Identifier("yo".to_string().into()),
                Token::NewLine,
                Token::Identifier("man".to_string().into()),
                Token::NewLine,
                Token::Dedent,
            ],
        )
    }

    #[test]
    fn test_indentation_multiple_up() {
        assert_tokens(
            r#"
hello
        world
        yo
        man 
"#,
            vec![
                Token::NewLine,
                Token::Identifier("hello".to_string().into()),
                Token::NewLine,
                Token::Indent,
                Token::Indent,
                Token::Identifier("world".to_string().into()),
                Token::NewLine,
                Token::Identifier("yo".to_string().into()),
                Token::NewLine,
                Token::Identifier("man".to_string().into()),
                Token::NewLine,
                Token::Dedent,
                Token::Dedent,
            ],
        )
    }

    #[test]
    fn test_indentation_multiple_down() {
        assert_tokens(
            r#"
        hello
world
"#,
            vec![
                Token::NewLine,
                Token::Indent,
                Token::Indent,
                Token::Identifier("hello".to_string().into()),
                Token::NewLine,
                Token::Dedent,
                Token::Dedent,
                Token::Identifier("world".to_string().into()),
                Token::NewLine,
            ],
        )
    }

    #[test]
    fn test_indentation_up_down() {
        assert_tokens(
            r#"
hello
    world
        yo
    man 
"#,
            vec![
                Token::NewLine,
                Token::Identifier("hello".to_string().into()),
                Token::NewLine,
                Token::Indent,
                Token::Identifier("world".to_string().into()),
                Token::NewLine,
                Token::Indent,
                Token::Identifier("yo".to_string().into()),
                Token::NewLine,
                Token::Dedent,
                Token::Identifier("man".to_string().into()),
                Token::NewLine,
                Token::Dedent,
            ],
        )
    }

    #[test]
    fn test_single_tokens() {
        assert_token(">", Token::Greater);
        assert_token("<", Token::Less);
        assert_token("=", Token::Equal);
        assert_token("!", Token::Bang);
        assert_token(":", Token::Colon);
        assert_token(",", Token::Comma);
        assert_token("(", Token::LParen);
        assert_token(")", Token::RParen);
        assert_token("{", Token::LBrace);
        assert_token("}", Token::RBrace);
        assert_token("[", Token::LBracket);
        assert_token("]", Token::RBracket);
        assert_token("+", Token::Plus);
        assert_token("-", Token::Minus);
        assert_token("/", Token::Slash);
        assert_token("*", Token::Asterisk);
        assert_token("%", Token::Modulo);
        assert_token(".", Token::Dot);
        assert_token("|", Token::Pipe);
    }

    #[test]
    fn test_multi_char_tokens() {
        assert_token(">=", Token::GreaterEqual);
        assert_token("<=", Token::LessEqual);
        assert_token("==", Token::EqualEqual);
        assert_token("!=", Token::BangEqual);
        assert_token("->", Token::ArrowRight);
    }

    #[test]
    fn test_string() {
        assert_token(r#""hello""#, Token::String("hello".to_string().into()));
    }

    #[test]
    fn test_comment() {
        assert_tokens("// hello", vec![]);
        assert_tokens("// hello\n", vec![Token::NewLine]);
    }

    #[test]
    fn test_identifier() {
        assert_token("hello", Token::Identifier("hello".to_string().into()));
    }

    #[test]
    fn test_integer() {
        assert_token("123", Token::Integer(123));
    }

    #[test]
    fn test_float() {
        assert_token("123.456", Token::Float(123.456));
    }

    #[test]
    fn test_eof() {
        assert_tokens("", vec![]);
    }

    #[test]
    fn test_multiple_tokens() {
        assert_tokens(
            "hello world",
            vec![
                Token::Identifier("hello".to_string().into()),
                Token::Identifier("world".to_string().into()),
            ],
        );
    }

    #[test]
    fn test_multiple_lines() {
        assert_tokens(
            "hello\nworld",
            vec![
                Token::Identifier("hello".to_string().into()),
                Token::NewLine,
                Token::Identifier("world".to_string().into()),
            ],
        );
    }

    #[test]
    fn test_multiple_lines_with_comment() {
        assert_tokens(
            "hello // world\nworld",
            vec![
                Token::Identifier("hello".to_string().into()),
                Token::NewLine,
                Token::Identifier("world".to_string().into()),
            ],
        );
    }

    #[test]
    fn test_infix_expression() {
        assert_tokens(
            "1 + 2",
            vec![Token::Integer(1), Token::Plus, Token::Integer(2)],
        );
    }

    #[test]
    fn test_prefix_expression() {
        assert_tokens("-1", vec![Token::Minus, Token::Integer(1)]);
    }
}
