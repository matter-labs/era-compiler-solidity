//!
//! EasyCrypt AST node containing a definition of a new variable.
//!

use crate::easycrypt::syntax::r#type::Type;
use crate::easycrypt::syntax::reference::Reference;
use crate::easycrypt::syntax::Name;

/// EasyCrypt AST node containing a definition of a new variable.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Definition {
    /// Name of the variable.
    pub identifier: Name,

    /// Location of the original variable in the source YUL file.
    pub location: Option<crate::yul::path::Path>,

    /// Type of the variable, if the definition is annotated with one.
    pub r#type: Option<Type>,
}

impl Definition {
    /// Produce a reference to the variable.
    pub fn reference(&self) -> Reference {
        Reference {
            identifier: self.identifier.clone(),
            location: self.location.clone(),
        }
    }
}
