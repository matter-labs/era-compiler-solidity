//!
//! EasyCrypt AST node containing a reference to a previously defined variable.
//!

use crate::easycrypt::syntax::Name;

/// EasyCrypt AST node containing a reference to a previously defined variable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Reference {
    /// Name of the variable
    pub identifier: Name,

    /// Location of the original variable in the source YUL file.
    pub location: Option<crate::yul::path::Path>,
}
