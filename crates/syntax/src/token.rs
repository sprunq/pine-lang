use base::located::Located;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Indent,
    UnIndent,
    Eof,
    /// `(`
    LParen,
    /// `)`
    RParen,
    /// `{`
    LBrace,
    /// `}`
    RBrace,
    /// `[`
    LBracket,
    /// `]`
    RBracket,
    /// `,`
    Comma,
    /// `.`
    Dot,
    /// `-`
    Minus,
    /// `+`
    Plus,
    /// `/`
    Slash,
    /// `%`
    Modulo,
    /// `*`
    Asterisk,
    /// `:`
    Colon,
    /// `->`
    ArrowRight,
    /// `|`
    Pipe,

    /// `!`
    Bang,
    /// `!=`
    BangEqual,
    /// `=`
    Equal,
    /// `==`
    EqualEqual,
    /// `>`
    Greater,
    /// `>=`
    GreaterEqual,
    /// `<`
    Less,
    /// `<=`
    LessEqual,

    /// `identifier`
    Identifier(String),
    /// a `"string"` literal
    String(String),
    /// a `123` literal
    Integer(u64),
    /// a `123.456` literal
    Float(f64),

    /// `and`
    And,
    /// `or`
    Or,
    /// `type`
    Type,
    /// `else`
    Else,
    /// `false`
    False,
    /// `true`
    True,
    /// `fun`
    Fun,
    /// `if`
    If,
    /// `return`
    Return,
    /// `self`
    TSelf,

    /// `let`
    Let,
    /// `loop`
    Loop,
    /// `break`
    Break,

    /// `bool`
    TyBool,
    /// `i8`
    TyI8,
    /// `i32`
    TyI32,
    /// `i64`
    TyI64,
    /// `u8`
    TyU8,
    /// `u32`
    TyU32,
    /// `u64`
    TyU64,
    /// `f32`
    TyF32,
    /// `f64`
    TyF64,
    /// `str`
    TyStr,
}

impl Token {
    pub fn lookup_keyword(ident: &str) -> Option<Token> {
        let kw = match ident {
            "and" => Token::And,
            "or" => Token::Or,
            "type" => Token::Type,
            "else" => Token::Else,
            "false" => Token::False,
            "fun" => Token::Fun,
            "if" => Token::If,
            "return" => Token::Return,
            "self" => Token::TSelf,
            "true" => Token::True,
            "let" => Token::Let,
            "loop" => Token::Loop,
            "break" => Token::Break,
            "bool" => Token::TyBool,
            "i8" => Token::TyI8,
            "i32" => Token::TyI32,
            "i64" => Token::TyI64,
            "u8" => Token::TyU8,
            "u32" => Token::TyU32,
            "u64" => Token::TyU64,
            "f32" => Token::TyF32,
            "f64" => Token::TyF64,
            "str" => Token::TyStr,
            _ => return None,
        };
        Some(kw)
    }
}
