//!
//! Attributes of YUL definitions, describing their effect on memory, storage and other context.
//!

use super::usage::Usage;

/// Attributes of a procedure. Describe aspects of its behavior that are relevant for transpilation.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Attributes {
    /// How is this procedure affecting heap.
    pub heap_user: Usage,
    /// How is this procedure affecting (permanent) storage.
    pub storage_user: Usage,
    /// How is this procedure affecting transient storage.
    pub transient_user: Usage,

    /// Other generic side effects
    pub other: Usage,
}

impl Attributes {
    pub fn heap(usage: Usage) -> Self {
        Self {
            heap_user: usage,
            ..Default::default()
        }
    }
    pub fn storage(usage: Usage) -> Self {
        Self {
            storage_user: usage,
            ..Default::default()
        }
    }
    pub fn transient(usage: Usage) -> Self {
        Self {
            transient_user: usage,
            ..Default::default()
        }
    }
    pub fn other(usage: Usage) -> Self {
        Self {
            other: usage,
            ..Default::default()
        }
    }

    pub fn union(&self, other: Self) -> Self {
        Self {
            heap_user: self.heap_user.union(&other.heap_user),
            storage_user: self.storage_user.union(&other.storage_user),
            transient_user: self.transient_user.union(&other.transient_user),
            other: self.other.union(&other.other),
        }
    }
}
