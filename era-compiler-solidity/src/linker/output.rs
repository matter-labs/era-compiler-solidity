//!
//! The linker output.
//!

use std::collections::BTreeMap;

///
/// The linker output.
///
#[derive(Debug, Default, serde::Serialize)]
pub struct Output {
    /// Linked bytecode files with bytecode hashes.
    pub linked: BTreeMap<String, String>,
    /// Unlinked bytecode files with the list of unlinked symbols.
    pub unlinked: BTreeMap<String, Vec<String>>,
    /// Ignored bytecode files that do not require linking.
    pub ignored: BTreeMap<String, String>,
}
