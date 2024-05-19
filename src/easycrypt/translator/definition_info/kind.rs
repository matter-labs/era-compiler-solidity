//!
//! Kind of a [`DefinitionInfo`].
//!

/// Kind of a [`DefinitionInfo`].
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Kind {
    Function,
    Procedure,
    Variable,
}
