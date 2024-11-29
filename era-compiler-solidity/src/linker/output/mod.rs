//!
//! The linker output.
//!

pub mod contract;
pub mod unlinked;

use std::collections::BTreeMap;

use self::contract::Contract;
use self::unlinked::Unlinked;

///
/// The linker output.
///
#[derive(Debug, Default, serde::Serialize)]
pub struct Output {
    /// Linked bytecode files with bytecode hashes.
    pub linked: BTreeMap<String, Contract>,
    /// Unlinked bytecode files with the list of unlinked symbols.
    pub unlinked: BTreeMap<String, Unlinked>,
    /// Ignored bytecode files that do not require linking.
    pub ignored: BTreeMap<String, Contract>,
}
