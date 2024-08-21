//!
//! Kind of a context that has to be passed to a function and returned from it,
//! as well as propagated through calls.
//!

use std::fmt::Display;

///
/// Kind of a context that has to be passed to a function and returned from it,
/// as well as propagated through calls.
///
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum ContextKind {
    Memory,
    Storage,
    TransientStorage,
    Other,
}

impl ContextKind {
    pub const ALL_KINDS: &'static [ContextKind] = &[
        ContextKind::Memory,
        ContextKind::Storage,
        ContextKind::TransientStorage,
        ContextKind::Other,
    ];
}
impl Display for ContextKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ContextKind::Memory => "mem",
            ContextKind::Storage => "storage",
            ContextKind::TransientStorage => "transient_storage",
            ContextKind::Other => "context",
        })
    }
}
