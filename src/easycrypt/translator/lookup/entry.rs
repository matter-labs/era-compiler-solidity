//!
//! An entry in a lookup table.
//!

use crate::easycrypt::syntax::Name;
use crate::yul::path::Path;

/// An entry in a lookup data structure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub kind: super::kind::Kind,
    pub name: Name,
    pub path: Path,
}
