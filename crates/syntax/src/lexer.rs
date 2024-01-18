use crate::token::Token;
use base::{located::Located, source_id::SourceId};
use internment::Intern;
use messages::message::Message;
use std::{cmp::Ordering, collections::VecDeque, sync::mpsc::Sender};

pub struct Lexer<'source> {
    pub input: &'source str,
    pub chars: Vec<char>,
    pub file_id: SourceId,
    ch_pos: usize,
    prev_line_indent: usize,
    buffer: VecDeque<Located<Token>>,
    msg_sender: Sender<Message>,
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
    pub fn new(file_id: SourceId, input: &'source str, msg_sender: Sender<Message>) -> Self {
        let mut chars = input.chars().collect::<Vec<_>>();
        chars.push('\0');

        Self {
            ch_pos: 0,
            prev_line_indent: 0,
            file_id,
            input,
            chars,
            buffer: VecDeque::new(),
            msg_sender,
        }
    }

    fn next_token(&mut self) -> Located<Token> {
        if let Some(t) = self.buffer.pop_front() {
            return t;
        }

        self.skip_whitespace_until_nl();

        let start_pos = self.ch_pos;
        let tok = match &self.ch() {
            '\0' => Token::Eof,
            '\n' => {
                self.advance();
                let line_start = self.ch_pos;
                self.skip_whitespace_until_nl();
                let indent = self.ch_pos - line_start;

                if indent % 4 != 0 {
                    panic!("Only indent steps of 4 allowed")
                }

                let prev_indent_steps = self.prev_line_indent / 4;
                let indent_steps = indent / 4;
                let steps_diff = prev_indent_steps.abs_diff(indent_steps);

                // we can only return one token at a time, so we need to buffer the extra indents
                let mut push_extra_indents_to_buffer = |token: Token| {
                    for i in 0..steps_diff - 1 {
                        let t = Located::new(
                            self.file_id,
                            line_start + 4 * i..line_start + 4 * i + 4,
                            token.clone(),
                        );
                        self.buffer.push_back(t);
                    }
                };

                match indent_steps.cmp(&prev_indent_steps) {
                    Ordering::Less => {
                        self.prev_line_indent = indent;
                        push_extra_indents_to_buffer(Token::UnIndent);
                        Token::UnIndent
                    }
                    Ordering::Equal => return self.next_token(),
                    Ordering::Greater => {
                        self.prev_line_indent = indent;
                        push_extra_indents_to_buffer(Token::Indent);
                        Token::Indent
                    }
                }
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
                    while !matches!(self.ch(), '\n' | '\0') {
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
                let mut string = String::new();
                self.advance();
                loop {
                    match self.ch() {
                        '"' => break,
                        '\0' | '\n' => panic!("unterminated string"),
                        '\\' => {
                            self.advance();
                            match self.ch() {
                                '"' => string.push('"'),
                                _ => panic!("invalid escape sequence"),
                            }
                        }
                        _ => string.push(self.ch()),
                    }
                    self.advance();
                }
                self.advance();
                Token::String(Intern::new(string))
            }
            _ => {
                if Self::is_letter(self.ch()) && self.ch() != '_' {
                    let ident = self.read_identifier();
                    let s = String::from_iter(ident);
                    Token::lookup_keyword(&s).unwrap_or(Token::Identifier(Intern::new(s)))
                } else if self.ch().is_ascii_digit() {
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
        Located::new(self.file_id, start_pos..self.ch_pos, tok)
    }

    fn is_letter(character: char) -> bool {
        character.is_alphabetic() || character == '_'
    }

    fn read_identifier(&mut self) -> &[char] {
        let start_pos = self.ch_pos;
        while Self::is_letter(self.ch()) || self.ch().is_ascii_digit() {
            self.advance();
        }
        let end_pos = self.ch_pos;
        &self.chars[start_pos..end_pos]
    }

    fn consume_number(&mut self) -> Number {
        let mut parts = vec![];

        while self.ch().is_ascii_digit() || self.ch() == '.' {
            parts.push(self.ch());
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

    #[inline]
    fn skip_whitespace_until_nl(&mut self) {
        while self.ch() != '\n' && self.ch().is_whitespace() {
            self.advance()
        }
    }

    #[inline]
    fn advance(&mut self) {
        self.ch_pos += 1;
    }

    #[inline]
    fn ch(&self) -> char {
        match self.chars.get(self.ch_pos) {
            Some(ch) => *ch,
            None => '\0',
        }
    }

    #[inline]
    fn advance_if(&mut self, ch: char) -> bool {
        if self.ch() == ch {
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
    use std::sync;

    use super::*;

    fn assert_token(input: &str, expected: Token) {
        let (msg_sender, _) = sync::mpsc::channel();
        let lexer = Lexer::new(SourceId::from_path(""), input, msg_sender);
        let tokens = lexer.collect::<Vec<_>>();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].value, expected);
    }

    fn assert_tokens(input: &str, expected: Vec<Token>) {
        let (msg_sender, _) = sync::mpsc::channel();
        let lexer = Lexer::new(SourceId::from_path(""), input, msg_sender);
        let tokens = lexer.collect::<Vec<_>>();
        assert_eq!(tokens.len(), expected.len());
        for (expected, token) in expected.into_iter().zip(tokens) {
            assert_eq!(token.value, expected);
        }
    }

    #[test]
    fn test_indentation_no_indent() {
        assert_tokens(
            r#"
hello
world
"#,
            vec![
                Token::Identifier("hello".to_string().into()),
                Token::Identifier("world".to_string().into()),
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
                Token::Identifier("hello".to_string().into()),
                Token::Indent,
                Token::Identifier("world".to_string().into()),
                Token::Identifier("yo".to_string().into()),
                Token::Identifier("man".to_string().into()),
                Token::UnIndent,
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
                Token::Identifier("hello".to_string().into()),
                Token::Indent,
                Token::Indent,
                Token::Identifier("world".to_string().into()),
                Token::Identifier("yo".to_string().into()),
                Token::Identifier("man".to_string().into()),
                Token::UnIndent,
                Token::UnIndent,
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
                Token::Indent,
                Token::Indent,
                Token::Identifier("hello".to_string().into()),
                Token::UnIndent,
                Token::UnIndent,
                Token::Identifier("world".to_string().into()),
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
                Token::Identifier("hello".to_string().into()),
                Token::Indent,
                Token::Identifier("world".to_string().into()),
                Token::Indent,
                Token::Identifier("yo".to_string().into()),
                Token::UnIndent,
                Token::Identifier("man".to_string().into()),
                Token::UnIndent,
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
    fn test_string_with_escaped_quote() {
        assert_token(
            r#""hello \"world\"!""#,
            Token::String("hello \"world\"!".to_string().into()),
        );
    }

    #[test]
    fn test_comment() {
        assert_tokens("// hello", vec![]);
        assert_tokens("// hello\n", vec![]);
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
