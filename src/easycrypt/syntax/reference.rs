use super::Name;

/// Reference to a previously defined variable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Reference {
    /// Name of the variable
    pub identifier: Name,

    /// Location of the original variable in the source YUL file.
    pub location: Option<crate::yul::path::Path>,
}

// impl Hash for Reference {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.identifier.hash(state);
//         self.location.hash(state);
//     }
// }
