//!
//! EasyCrypt AST node containing a literal: string, bool, or an integer in
//! decimal or hexadecimal form.
//!
pub mod integer;

use self::integer::IntegerLiteral;

/// EasyCrypt AST node containing a literal: string, bool, or an integer in
/// decimal or hexadecimal form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    /// String literal, like `"hello"`.
    String(String),
    /// Integer literal, like `123` or `0x123`.
    Int(IntegerLiteral),
    /// Boolean literal, like `false`.
    Bool(bool),
}
