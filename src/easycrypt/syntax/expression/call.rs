//!
//! EasyCrypt AST node containing a call to a function (not procedure).
//!

use crate::easycrypt::syntax::expression::Expression;
use crate::easycrypt::syntax::function::name::FunctionName;

///
/// EasyCrypt AST node containing a call to a function (not procedure).
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionCall {
    pub target: FunctionName,
    pub arguments: Vec<Expression>,
}
