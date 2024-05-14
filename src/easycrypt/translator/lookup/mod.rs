//!
//! Lookup table for user-defined functions, variables and procedures. It
//! matches the name of a function/variable/procedure to its path from the root
//! of YUL syntax tree.
//!

pub mod entry;
pub mod kind;
pub mod stack_lookup;

pub use self::entry::Entry;

use crate::easycrypt::syntax::Name;
use crate::yul::path::Path;

/// Lookup map for user-defined functions, variables and procedures. It matches
/// the name of a function/variable/procedure to its path from the root of YUL
/// syntax tree.
pub trait ILookup {
    /// Add a variable to the topmost lexical scope.
    fn add_var(&mut self, name: &Name, path: &Path);

    /// Add a procedure to the topmost lexical scope.
    fn add_proc(&mut self, name: &Name, path: &Path);

    /// Add a function to the topmost lexical scope.
    fn add_fun(&mut self, name: &Name, path: &Path);

    /// Enter a new lexical scope.
    fn enter(&mut self);

    /// Leave the topmost lexical scope.
    fn leave(&mut self);

    /// Get an [`Entry`] by the entity name.
    fn get(&self, name: &Name) -> Option<Entry>;
}
