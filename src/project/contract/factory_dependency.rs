//!
//! The factory dependency trait.
//!

use std::collections::HashSet;

///
/// The factory dependency trait.
///
pub trait FactoryDependency {
    ///
    /// Returns path references.
    ///
    fn get_factory_dependencies(&self) -> HashSet<&str>;

    ///
    /// Drains factory dependencies.
    ///
    fn drain_factory_dependencies(&mut self) -> HashSet<String>;

    ///
    /// Whether the dependencies are satisfied.
    ///
    fn are_factory_dependencies_satisfied<D>(
        &self,
        evaluated_dependencies: Vec<&String>,
        resolver: &D,
    ) -> bool
    where
        D: era_compiler_llvm_context::Dependency;
}
