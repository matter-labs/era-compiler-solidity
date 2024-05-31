//!
//! Lookup table for user-defined functions, variables and procedures. It
//! matches the name of a function/variable/procedure to its path from the root
//! of YUL syntax tree.
//!

pub mod stack_impl;

/// Lookup map for user-defined functions, variables and procedures. It matches
/// the name of a function/variable/procedure to its path from the root of YUL
/// syntax tree.
pub trait IEnvironment<K, V>
where
    K: Clone + std::fmt::Debug + Eq + PartialEq,
    V: Clone + std::fmt::Debug + Eq + PartialEq,
{
    /// Add a variable to the topmost lexical scope.
    fn add(&mut self, name: &K, value: &V);

    /// Enter a new lexical scope.
    fn enter(&mut self);

    /// Leave the topmost lexical scope.
    fn leave(&mut self);

    /// Get an entry by its name.
    fn get(&self, name: &K) -> Option<V>;
}
