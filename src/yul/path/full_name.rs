use crate::yul::path::Path;

///
/// Fully qualified name of a YUL variable or function.
///
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct FullName {
    /// The name as given in the source code.
    pub name: String,
    /// The path to the definition, incorporating all lexical blocks starting
    /// from the root of the YUL syntax tree.
    pub path: Path,
}

impl FullName {
    ///
    /// Create a new instance of [`FullName`].
    ///
    pub fn new(name: String, path: Path) -> Self {
        Self { name, path }
    }
}
