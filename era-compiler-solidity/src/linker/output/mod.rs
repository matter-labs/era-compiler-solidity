//!
//! The linker output.
//!

pub mod contract;

use std::collections::BTreeMap;

use self::contract::Contract;

///
/// The linker output.
///
#[derive(Debug, Default, serde::Serialize)]
pub struct Output {
    /// Linked bytecode files with bytecode hashes.
    pub linked: BTreeMap<String, Contract>,
    /// Unlinked bytecode files with the list of unlinked symbols.
    pub unlinked: BTreeMap<String, Vec<String>>,
    /// Ignored bytecode files that do not require linking.
    pub ignored: BTreeMap<String, Contract>,
}
