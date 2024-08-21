//!
//! EasyCrypt AST node containing an `if` statement.
//!

use crate::easycrypt::syntax::expression::Expression;
use crate::easycrypt::syntax::statement::Statement;

///
/// EasyCrypt AST node containing an `if` statement.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfConditional {
    /// On which condition the statement should be executed.
    pub condition: Expression,
    /// Statement to be executed if the condition is true.
    pub yes: Box<Statement>,
    /// Optionally, a statement to be executed if the condition is false.
    pub no: Option<Box<Statement>>,
}
