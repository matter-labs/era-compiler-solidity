//!
//! Types of binary operations in EasyCrypt AST.
//!

/// Types of binary operations in EasyCrypt AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOpType {
    /// `x + y`.
    Add,
    /// `x - y`.
    Sub,
    /// `x * y`.
    Mul,
    /// `x / y` or `0` if `y == 0`.
    Div,
    /// `x % y` or `0` if `y == 0`.
    Mod,

    /// `1` if `x == y`, `0` otherwise.
    Eq,

    /// bitwise "or" of `x` and `y`.
    Or,
    /// bitwise "xor" of `x` and `y`.
    Xor,
    /// bitwise "and" of `x` and `y`.
    And,
    /// logical shift left `y` by `x` bits
    Shl,
    /// logical shift right `y` by `x` bits
    Shr,
    /// `x` to the power of `y`
    Exp,
}
