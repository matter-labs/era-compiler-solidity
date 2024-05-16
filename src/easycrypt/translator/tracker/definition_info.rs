//!
//! The path to a definition in YUL syntax tree.
//!

use crate::easycrypt::syntax::Name;
use crate::yul::path::Path;

use super::kind::Kind;

/// A definition of a variable, function or procedure, belonging to the current
/// lexical scope or one of its parents, as tracked by an instance of [`Tracker`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefinitionInfo {
    /// Kind of a definition: procedure, function or a variable.
    pub kind: Kind,
    /// Name of a definition, as given in the source code.
    pub name: Name,
    /// Path to the definition from the root of YUL syntax tree to its lexical
    /// scope. Contains all its parent lexical scopes.
    pub path: Path,
}
