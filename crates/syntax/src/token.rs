use internment::Intern;

#[derive(Clone, Copy, Debug, PartialEq, derive_more::Display)]
pub enum Token {
    #[display(fmt = "indent")]
    Indent,

    #[display(fmt = "unindent")]
    UnIndent,

    #[display(fmt = "newline")]
    NewLine,

    #[display(fmt = "EOF")]
    Eof,

    /// `(`
    #[display(fmt = "(")]
    LParen,

    /// `)`
    #[display(fmt = ")")]
    RParen,

    /// `{`
    #[display(fmt = "{{")]
    LBrace,

    /// `}`
    #[display(fmt = "}}")]
    RBrace,

    /// `[`
    #[display(fmt = "[")]
    LBracket,

    /// `]`
    #[display(fmt = "]")]
    RBracket,

    /// `,`
    #[display(fmt = ",")]
    Comma,

    /// `.`
    #[display(fmt = ".")]
    Dot,

    /// `-`
    #[display(fmt = "-")]
    Minus,

    /// `+`
    #[display(fmt = "+")]
    Plus,

    /// `/`
    #[display(fmt = "/")]
    Slash,

    /// `%`
    #[display(fmt = "%")]
    Modulo,

    /// `*`
    #[display(fmt = "*")]
    Asterisk,

    /// `:`
    #[display(fmt = ":")]
    Colon,

    /// `->`
    #[display(fmt = "->")]
    ArrowRight,

    /// `|`
    #[display(fmt = "|")]
    Pipe,

    /// `!`
    #[display(fmt = "!")]
    Bang,

    /// `!=`
    #[display(fmt = "!=")]
    BangEqual,

    /// `=`
    #[display(fmt = "=")]
    Equal,

    /// `==`
    #[display(fmt = "==")]
    EqualEqual,

    /// `>`
    #[display(fmt = ">")]
    Greater,

    /// `>=`
    #[display(fmt = ">=")]
    GreaterEqual,

    /// `<`
    #[display(fmt = "<")]
    Less,

    /// `<=`
    #[display(fmt = "<=")]
    LessEqual,

    /// `identifier`
    #[display(fmt = "{}", "_0")]
    Identifier(Intern<String>),

    /// a `"string"` literal
    #[display(fmt = "\"{}\"", "_0")]
    String(Intern<String>),

    /// a `123` literal
    #[display(fmt = "{}", "_0")]
    Integer(u64),

    /// a `123.456` literal
    #[display(fmt = "{}", "_0")]
    Float(f64),

    /// `and`
    #[display(fmt = "and")]
    And,

    /// `or`
    #[display(fmt = "or")]
    Or,

    /// `type`
    #[display(fmt = "type")]
    Type,

    /// `else`
    #[display(fmt = "else")]
    Else,

    /// `false`
    #[display(fmt = "false")]
    False,

    /// `true`
    #[display(fmt = "true")]
    True,

    /// `fun`
    #[display(fmt = "fun")]
    Fun,

    /// `if`
    #[display(fmt = "if")]
    If,

    /// `return`
    #[display(fmt = "return")]
    Return,

    /// `self`
    #[display(fmt = "self")]
    TSelf,

    /// `let`
    #[display(fmt = "let")]
    Let,

    /// `loop`
    #[display(fmt = "loop")]
    Loop,

    /// `break`
    #[display(fmt = "break")]
    Break,

    /// `bool`
    #[display(fmt = "bool")]
    TyBool,

    /// `i8`
    #[display(fmt = "i8")]
    TyI8,

    /// `i32`
    #[display(fmt = "i32")]
    TyI32,

    /// `i64`
    #[display(fmt = "i64")]
    TyI64,

    /// `u8`
    #[display(fmt = "u8")]
    TyU8,

    /// `u32`
    #[display(fmt = "u32")]
    TyU32,

    /// `u64`
    #[display(fmt = "u64")]
    TyU64,

    /// `f32`
    #[display(fmt = "f32")]
    TyF32,

    /// `f64`
    #[display(fmt = "f64")]
    TyF64,

    /// `str`
    #[display(fmt = "str")]
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
