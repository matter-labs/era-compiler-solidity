//!
//! EasyCrypt AST node containing a definition of a function.
//!

pub mod name;

use crate::yul::path::Path;

use self::name::FunctionName;

use crate::easycrypt::syntax::expression::Expression;
use crate::easycrypt::syntax::signature::Signature;

/// EasyCrypt AST node containing a definition of a function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    /// Name of the function.
    pub name: FunctionName,
    /// Optionally, a location of the original function in the YUL source code.
    pub location: Option<Path>,
    /// Function signature.
    pub signature: Signature,
    /// Function body, which can only be a single expression.
    pub body: Expression,
}
