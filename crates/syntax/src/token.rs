use logos::Logos;
use std::fmt::Display;
use std::num::{ParseFloatError, ParseIntError};

#[repr(u8)]
#[derive(Clone, Debug, Logos, PartialEq)]
pub enum Token {
    // Single-character tokens.
    #[token("(")]
    ParenOpen,
    #[token(")")]
    ParenClose,
    #[token("{")]
    BraceOpen,
    #[token("}")]
    BraceClose,
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

impl Into<String> for Token {
    fn into(self) -> String {
        self.to_string()
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Token::*;
        match self {
            ParenOpen => write!(f, "("),
            ParenClose => write!(f, ")"),
            BraceOpen => write!(f, "{{"),
            BraceClose => write!(f, "}}"),
            Comma => write!(f, ","),
            Dot => write!(f, "."),
            Minus => write!(f, "-"),
            Plus => write!(f, "+"),
            Semicolon => write!(f, ";"),
            Slash => write!(f, "/"),
            Modulo => write!(f, "%"),
            Asterisk => write!(f, "*"),
            Colon => write!(f, ":"),
            ArrowRight => write!(f, "->"),
            Bang => write!(f, "!"),
            BangEqual => write!(f, "!="),
            Equal => write!(f, "="),
            EqualEqual => write!(f, "=="),
            Greater => write!(f, ">"),
            GreaterEqual => write!(f, ">="),
            Less => write!(f, "<"),
            LessEqual => write!(f, "<="),
            Identifier(s) => write!(f, "{}", s),
            String(s) => write!(f, "\"{}\"", s),
            Integer(i) => write!(f, "{}", i),
            Float(fl) => write!(f, "{}", fl),
            And => write!(f, "and"),
            Type => write!(f, "type"),
            Else => write!(f, "else"),
            False => write!(f, "false"),
            Fun => write!(f, "fun"),
            If => write!(f, "if"),
            Or => write!(f, "or"),
            Return => write!(f, "return"),
            TSelf => write!(f, "self"),
            True => write!(f, "true"),
            Let => write!(f, "let"),
            Loop => write!(f, "loop"),
            Break => write!(f, "break"),
            TyBool => write!(f, "bool"),
            TyI8 => write!(f, "i8"),
            TyI32 => write!(f, "i32"),
            TyI64 => write!(f, "i64"),
            TyU8 => write!(f, "u8"),
            TyU32 => write!(f, "u32"),
            TyU64 => write!(f, "u64"),
            TyF32 => write!(f, "f32"),
            TyF64 => write!(f, "f64"),
            TyStr => write!(f, "str"),
            Error => write!(f, "error"),
        }
    }
}

impl Token {
    pub fn id(&self) -> u8 {
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}
