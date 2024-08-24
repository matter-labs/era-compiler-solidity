//!
//! Path tracker for the EasyCrypt to Yul translation.
//!

use crate::data_structures::environment::stack_impl::Environment;
use crate::data_structures::environment::IEnvironment;
use crate::yul::path::builder::Builder;
use crate::yul::path::tracker::PathTracker;
use crate::yul::path::Path;

///
/// Path tracker with a symbol table.
///
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SymbolTracker<T>
where
    T: Clone + std::fmt::Debug + PartialEq + Eq,
{
    /// Lookup table for user-defined functions, variables and procedures. It
    /// matches the name of a function/variable/procedure to its path from the root
    /// of YUL syntax tree.
    symbols: Environment<String, T>,

    /// Tracker of the current path from the root of the YUL syntax tree.
    location: Builder,
}

impl<T> PathTracker for SymbolTracker<T>
where
    T: Clone + std::fmt::Debug + PartialEq + Eq,
{
    fn here(&self) -> &Path {
        self.location.here()
    }

    fn leave(&mut self) {
        self.location.leave();
        self.symbols.leave()
    }

    fn enter_block(&mut self) {
        self.location.enter_block();
        self.symbols.enter()
    }

    fn enter_function(&mut self, ident: &str) {
        self.location.enter_function(ident);
        self.symbols.enter()
    }

    fn enter_code(&mut self) {
        self.location.enter_code();
        self.symbols.enter()
    }

    fn enter_object(&mut self, ident: &str) {
        self.location.enter_object(ident);
        self.symbols.enter()
    }

    fn enter_if_cond(&mut self) {
        self.location.enter_if_cond();
        self.symbols.enter()
    }

    fn enter_if_then(&mut self) {
        self.location.enter_if_then();
        self.symbols.enter()
    }

    fn enter_for1(&mut self) {
        self.location.enter_for1();
        self.symbols.enter()
    }

    fn enter_for2(&mut self) {
        self.location.enter_for2();
        self.symbols.enter()
    }

    fn enter_for3(&mut self) {
        self.location.enter_for3();
        self.symbols.enter()
    }
}

impl<T> SymbolTracker<T>
where
    T: Clone + std::fmt::Debug + PartialEq + Eq,
{
    pub fn add(&mut self, name: &String, value: &T) {
        self.symbols.add(name, value);
    }

    /// Creates a new, empty instance of [`Tracker`]
    pub fn new() -> Self {
        Self {
            symbols: Environment::new(),
            location: Builder::new(Path::empty()),
        }
    }
}

impl<T> SymbolTracker<T>
where
    T: Clone + std::fmt::Debug + PartialEq + Eq,
{
    pub fn get(&self, name: &String) -> Option<T> {
        self.symbols.get(name)
    }
}
