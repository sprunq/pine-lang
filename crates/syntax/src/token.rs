use logos::Logos;
use std::num::{ParseFloatError, ParseIntError};

#[derive(Clone, Debug, Logos, PartialEq)]
pub enum Token {
    // Single-character tokens.
    #[token("(")]
    LtParen,
    #[token(")")]
    RtParen,
    #[token("{")]
    LtBrace,
    #[token("}")]
    RtBrace,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("-")]
    Minus,
    #[token("+")]
    Plus,
    #[token(";")]
    Semicolon,
    #[token("/")]
    Slash,
    #[token("%")]
    Modulo,
    #[token("*")]
    Asterisk,
    #[token(":")]
    Colon,
    #[token("->")]
    ArrowRight,

    // One or two character tokens.
    #[token("!")]
    Bang,
    #[token("!=")]
    BangEqual,
    #[token("=")]
    Equal,
    #[token("==")]
    EqualEqual,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEqual,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEqual,

    // Literals.
    #[regex(r#"(\p{XID_Start}|_)\p{XID_Continue}*"#, lex_identifier)]
    Identifier(String),
    #[regex(r#""[^"]*""#, lex_string)]
    String(String),
    #[regex("[0-9][0-9_]*", lex_int_dec)]
    #[regex(r"0[xX][a-fA-F0-9][a-fA-F0-9_]*", lex_int_hex)]
    #[regex("0[bB][0-1][0-1_]*", lex_int_bin)]
    Integer(i64),
    #[regex(r#"[0-9]+(\.[0-9]+)+"#, lex_float)]
    Float(f64),

    // Keywords.
    #[token("and")]
    And,
    #[token("type")]
    Type,
    #[token("else")]
    Else,
    #[token("false")]
    False,
    #[token("fun")]
    Fun,
    #[token("if")]
    If,
    #[token("or")]
    Or,
    #[token("return")]
    Return,
    #[token("self")]
    TSelf,
    #[token("true")]
    True,
    #[token("let")]
    Let,
    #[token("loop")]
    Loop,
    #[token("break")]
    Break,

    // types
    #[token("bool")]
    TyBool,
    #[token("i8")]
    TyI8,
    #[token("i32")]
    TyI32,
    #[token("i64")]
    TyI64,
    #[token("u8")]
    TyU8,
    #[token("u32")]
    TyU32,
    #[token("u64")]
    TyU64,
    #[token("f32")]
    TyF32,
    #[token("f64")]
    TyF64,
    #[token("str")]
    TyStr,

    #[regex(r"//.*", logos::skip)]
    #[regex(r"[ \r\n\t\f]+", logos::skip)]
    #[error]
    Error,
}

fn lex_int_dec(lexer: &mut logos::Lexer<Token>) -> Result<i64, ParseIntError> {
    let slice = lexer.slice();
    let without_prefix = slice.trim_start_matches("0d");
    let without_underscores = without_prefix.replace('_', "");
    without_underscores.parse::<i64>()
}

fn lex_int_hex(lexer: &mut logos::Lexer<Token>) -> Result<i64, ParseIntError> {
    let slice = lexer.slice();
    let without_prefix = slice.trim_start_matches("0x");
    let without_underscores = without_prefix.replace('_', "");
    i64::from_str_radix(&without_underscores, 16)
}

fn lex_int_bin(lexer: &mut logos::Lexer<Token>) -> Result<i64, ParseIntError> {
    let slice = lexer.slice();
    let without_prefix = slice.trim_start_matches("0b");
    let without_underscores = without_prefix.replace('_', "");
    i64::from_str_radix(&without_underscores, 2)
}

fn lex_float(lexer: &mut logos::Lexer<Token>) -> Result<f64, ParseFloatError> {
    let slice = lexer.slice();
    slice.parse::<f64>()
}

fn lex_string(lexer: &mut logos::Lexer<Token>) -> String {
    let slice = lexer.slice();
    slice[1..slice.len() - 1].to_string()
}

fn lex_identifier(lexer: &mut logos::Lexer<Token>) -> String {
    let slice = lexer.slice();
    slice.to_string()
}
