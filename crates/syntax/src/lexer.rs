use crate::token::Token;
use base::{located::Located, source_id::SourceId};
use std::str::Chars;

pub struct Lexer<'source> {
    pub input: &'source str,
    pub iter: Chars<'source>,
    pub file_id: SourceId,
    pos: usize,
    ch: char,
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Located<Token>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Located {
                value: Token::Eof, ..
            } => None,
            token => Some(token),
        }
    }
}

impl<'source> Lexer<'source> {
    pub fn new(file_id: SourceId, input: &'source str) -> Self {
        let mut iter = input.chars();
        Self {
            ch: iter.next().unwrap_or('\0'),
            pos: 0,
            file_id,
            input,
            iter,
        }
    }

    pub fn next_token(&mut self) -> Located<Token> {
        self.skip_whitespace();
        let tok: Token;
        let start_pos = self.pos;
        tok = match &self.ch {
            '\0' => Token::Eof,
            ':' => Token::Colon,
            ',' => Token::Comma,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            '=' => {
                if self.advance_if_peeked('=') {
                    Token::EqualEqual
                } else {
                    Token::Equal
                }
            }
            '!' => {
                if self.advance_if_peeked('=') {
                    Token::BangEqual
                } else {
                    Token::Bang
                }
            }
            '>' => {
                if self.advance_if_peeked('=') {
                    Token::GreaterEqual
                } else {
                    Token::Greater
                }
            }
            '<' => {
                if self.advance_if_peeked('=') {
                    Token::LessEqual
                } else {
                    Token::Less
                }
            }
            '+' => Token::Plus,
            '-' => {
                if self.advance_if_peeked('>') {
                    Token::ArrowRight
                } else {
                    Token::Minus
                }
            }
            '/' => {
                // line comment
                if self.advance_if_peeked('/') {
                    while !matches!(self.ch, '\n' | '\0') {
                        self.advance();
                    }
                    return self.next_token();
                } else {
                    Token::Slash
                }
            }
            '*' => Token::Asterisk,
            '%' => Token::Modulo,
            '.' => Token::Dot,
            '|' => Token::Pipe,
            '"' => {
                let mut string = String::new();
                self.advance();
                loop {
                    match self.ch {
                        '"' => break,
                        '\0' | '\n' => panic!("unterminated string"),
                        '\\' => {
                            self.advance();
                            match self.ch {
                                '"' => string.push('"'),
                                _ => panic!("invalid escape sequence"),
                            }
                        }
                        _ => string.push(self.ch),
                    }
                    self.advance();
                }
                self.advance();
                Token::String(string)
            }
            _ => {
                if Self::is_letter(self.ch) && self.ch != '_' {
                    let ident = self.read_identifier();
                    Token::lookup(ident)
                } else if self.ch.is_ascii_digit() {
                    let number = self.consume_number();
                    match number {
                        Number::Int(i) => Token::Integer(i),
                        Number::Float(f) => Token::Float(f),
                    }
                } else {
                    panic!()
                }
            }
        };
        self.advance();
        Located::new(self.file_id, start_pos..self.pos, tok)
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.advance();
        }
    }

    fn is_letter(character: char) -> bool {
        character.is_alphabetic() || character == '_'
    }

    pub fn read_identifier(&mut self) -> &str {
        let start_pos = self.pos;
        while Self::is_letter(self.ch) || self.ch.is_ascii_digit() {
            self.advance();
        }
        let end_pos = self.pos;
        &self.input[start_pos..end_pos]
    }

    fn consume_number(&mut self) -> Number {
        let mut parts = vec![];

        while self.ch.is_ascii_digit() || self.ch == '.' {
            parts.push(self.ch);
            self.advance();
        }

        let number = parts.iter().collect::<String>();

        if number.contains('.') {
            let number = number.parse::<f64>().unwrap();
            Number::Float(number)
        } else {
            let number = number.parse::<u64>().unwrap();
            Number::Int(number)
        }
    }

    fn advance(&mut self) {
        self.ch = self.iter.next().unwrap_or('\0');
        self.pos += 1;
    }

    fn peek_char(&mut self) -> char {
        self.iter.clone().next().unwrap_or('\0')
    }

    fn advance_if_peeked(&mut self, ch: char) -> bool {
        if self.peek_char() == ch {
            self.advance();
            true
        } else {
            false
        }
    }
}

enum Number {
    Int(u64),
    Float(f64),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_token(input: &str, expected: Token) {
        let mut lexer = Lexer::new(SourceId::from_path(""), input);
        let token = lexer.next_token();
        assert_eq!(token.value, expected);
        let empty = lexer.next_token();
        assert_eq!(empty.value, Token::Eof);
    }

    fn assert_tokens(input: &str, expected: Vec<Token>) {
        let mut lexer = Lexer::new(SourceId::from_path(""), input);
        for expected in expected {
            let token = lexer.next_token();
            assert_eq!(token.value, expected);
        }
        let empty = lexer.next_token();
        assert_eq!(empty.value, Token::Eof);
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
        assert_token(r#""hello""#, Token::String("hello".to_string()));
    }

    #[test]
    fn test_string_with_escaped_quote() {
        assert_token(
            r#""hello \"world\"!""#,
            Token::String("hello \"world\"!".to_string()),
        );
    }

    #[test]
    fn test_comment() {
        assert_token("// hello", Token::Eof);
        assert_token("// hello\n", Token::Eof);
    }

    #[test]
    fn test_identifier() {
        assert_token("hello", Token::Identifier("hello".to_string()));
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
        assert_token("", Token::Eof);
    }

    #[test]
    fn test_whitespace() {
        assert_token("  \n  ", Token::Eof);
        assert_token("  \n  \n  ", Token::Eof);
    }

    #[test]
    fn test_multiple_tokens() {
        assert_tokens(
            "hello world",
            vec![
                Token::Identifier("hello".to_string()),
                Token::Identifier("world".to_string()),
            ],
        );
    }

    #[test]
    fn test_multiple_lines() {
        assert_tokens(
            "hello\nworld",
            vec![
                Token::Identifier("hello".to_string()),
                Token::Identifier("world".to_string()),
            ],
        );
    }

    #[test]
    fn test_multiple_lines_with_comment() {
        assert_tokens(
            "hello // world\nworld",
            vec![
                Token::Identifier("hello".to_string()),
                Token::Identifier("world".to_string()),
            ],
        );
    }

    #[test]
    fn test_multiple_lines_with_comment_and_whitespace() {
        assert_tokens(
            "hello // world\n\nworld",
            vec![
                Token::Identifier("hello".to_string()),
                Token::Identifier("world".to_string()),
            ],
        );
    }

    #[test]
    fn test_infix_expression() {
        assert_tokens(
            "1 + 2",
            vec![
                Token::Integer(1),
                Token::Plus,
                Token::Integer(2),
                Token::Eof,
            ],
        );
    }

    #[test]
    fn test_prefix_expression() {
        assert_tokens("-1", vec![Token::Minus, Token::Integer(1), Token::Eof]);
    }
}
