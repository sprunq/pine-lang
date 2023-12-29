#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum CUnaryOperator {
    /// `operand++`
    PostIncrement,
    /// `operand--`
    PostDecrement,
    /// `++operand`
    PreIncrement,
    /// `--operand`
    PreDecrement,
    /// `&operand`
    Address,
    /// `*operand`
    Indirection,
    /// `+operand`
    Plus,
    /// `-operand`
    Minus,
    /// `~operand`
    Complement,
    /// `!operand`
    Negate,
}

/// All operators with two operands
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum CBinaryOperator {
    /// `lhs[rhs]`
    Index,
    /// `lhs * rhs`
    Multiply,
    /// `lhs / rhs`
    Divide,
    /// `lhs % rhs`
    Modulo,
    /// `lhs + rhs`
    Plus,
    /// `lhs - rhs`
    Minus,
    /// `lhs << rhs`
    ShiftLeft,
    /// `lhs >> rhs`
    ShiftRight,
    /// `lhs < rhs`
    Less,
    /// `lhs > rhs`
    Greater,
    /// `lhs <= rhs`
    LessOrEqual,
    /// `lhs >= rhs`
    GreaterOrEqual,
    /// `lhs == rhs`
    Equals,
    /// `lhs != rhs`
    NotEquals,
    /// `lhs & rhs`
    BitwiseAnd,
    /// `lhs ^ rhs`
    BitwiseXor,
    /// `lhs | rhs`
    BitwiseOr,
    /// `lhs && rhs`
    LogicalAnd,
    /// `lhs || rhs`
    LogicalOr,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CAssignmentOperator {
    /// `lhs = rhs`
    Assign,
    /// `lhs *= rhs`
    AssignMultiply,
    /// `lhs /= rhs`
    AssignDivide,
    /// `lhs %= rhs`
    AssignModulo,
    /// `lhs += rhs`
    AssignPlus,
    /// `lhs -= rhs`
    AssignMinus,
    /// `lhs <<= rhs`
    AssignShiftLeft,
    /// `lhs >>= rhs`
    AssignShiftRight,
    /// `lhs &= rhs`
    AssignBitwiseAnd,
    /// `lhs ^= rhs`
    AssignBitwiseXor,
    /// `lhs |= rhs`
    AssignBitwiseOr,
}
