use serde::Serialize;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize)]
pub enum OpInfix {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
    LogicAnd,
    LogicOr,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize)]
pub enum OpPrefix {
    Negate,
    Not,
}
