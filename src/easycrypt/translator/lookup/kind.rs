//!
//! Kind of an entry in the lookup table.
//!

/// Kind of an entry in the lookup table.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    Proc,
    Function,
    Variable,
}
