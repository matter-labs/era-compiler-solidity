use super::r#type::Type;
use super::reference::Reference;
use super::Name;

/// Definition of a new variable
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

// impl Hash for Definition {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.identifier.hash(state);
//         self.location.hash(state);
//         self.r#type.hash(state);
//     }
// }
