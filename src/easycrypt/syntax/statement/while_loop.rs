//!
//! EasyCrypt AST node containing a `while` loop.
//!

use crate::easycrypt::syntax::expression::Expression;
use crate::easycrypt::syntax::statement::Statement;

///
/// EasyCrypt AST node containing a `while` loop.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhileLoop {
    /// Execute while this condition is true.
    pub condition: Expression,
    /// Execute these statements while the condition is true.
    pub body: Box<Statement>,
}
