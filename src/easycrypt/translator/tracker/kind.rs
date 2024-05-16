//!
//! Kind of an entry in the symbol table used by tracker.
//!

/// Kind of an entry in the symbol table used by tracker.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    /// The entry contains a procedure.
    Proc,
    /// The entry contains a function.
    Function,
    /// The entry contains a variable.
    Variable,
}
