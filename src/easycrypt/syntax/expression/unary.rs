#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOpType {
    Neg,
    /// bitwise "not" of `x` (every bit of `x` is negated).
    Not,
}
