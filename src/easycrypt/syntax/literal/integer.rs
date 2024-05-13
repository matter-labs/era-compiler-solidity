//!
//! EasyCrypt AST node containing an integer literal in decimal or hexadecimal form.
//!

/// EasyCrypt AST node containing an integer literal in decimal or hexadecimal form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegerLiteral {
    /// Integer literal in decimal form, like `123`.
    Decimal { inner: String },
    /// Integer literal in hexadecimal form, like `0x123`.
    Hexadecimal { inner: String },
}
