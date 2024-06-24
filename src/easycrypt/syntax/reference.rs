//!
//! EasyCrypt AST node containing a reference to a previously defined variable.
//!

use crate::{easycrypt::syntax::Name, yul::path::full_name::FullName};

/// EasyCrypt AST node containing a reference to a previously defined variable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Reference {
    /// Name of the variable
    pub identifier: Name,

    /// Location of the original variable in the source YUL file.
    pub location: Option<crate::yul::path::Path>,
}

impl From<&FullName> for Reference {
    fn from(value: &FullName) -> Self {
        Self {
            identifier: value.name.clone(),
            location: Some(value.path.clone()),
        }
    }
}
