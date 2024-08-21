//!
//! Types of unary operations in EasyCrypt AST.
//!

///
/// Types of unary operations in EasyCrypt AST.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOpType {
    /// Logic negation (`Neg x` is true if and only if `x` is false).
    Neg,
    /// Bitwise "not" of `x` (every bit of `x` is negated).
    Not,
}
