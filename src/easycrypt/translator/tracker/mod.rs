//!
//! Path tracker for the EasyCrypt to Yul translation.
//!

pub mod definition_info;
pub mod kind;

use crate::easycrypt::syntax::Name;
use crate::yul::path::tracker::PathTracker;
use crate::yul::path::Builder;
use crate::yul::path::Path;

use crate::data_structures::symbol_table::stack_impl::SymbolTable;
use crate::data_structures::symbol_table::ISymbolTable;

use self::definition_info::DefinitionInfo;
use self::kind::Kind;

/// Path tracker for the EasyCrypt to Yul translation.
#[derive(Clone, Debug)]
pub struct Tracker {
    /// Lookup table for user-defined functions, variables and procedures. It
    /// matches the name of a function/variable/procedure to its path from the root
    /// of YUL syntax tree.
    symbols: SymbolTable<Name, DefinitionInfo>,

    /// Tracker of the current path from the root of the YUL syntax tree.
    location: Builder,
}

impl PathTracker for Tracker {
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

impl Tracker {
    pub fn add_var(&mut self, name: &Name) {
        self.symbols.add(
            name,
            &DefinitionInfo {
                kind: Kind::Variable,
                name: name.clone(),
                path: self.here().clone(),
            },
        )
    }

    pub fn add_proc(&mut self, name: &Name) {
        self.symbols.add(
            name,
            &DefinitionInfo {
                kind: Kind::Proc,
                name: name.clone(),
                path: self.here().clone(),
            },
        )
    }

    pub fn add_fun(&mut self, name: &Name) {
        self.symbols.add(
            name,
            &DefinitionInfo {
                kind: Kind::Function,
                name: name.clone(),
                path: self.here().clone(),
            },
        )
    }

    pub fn get(&self, name: &Name) -> Option<DefinitionInfo> {
        self.symbols.get(name)
    }

    /// Creates a new, empty instance of [`Tracker`]
    pub fn new() -> Self {
        Self {
            symbols: SymbolTable::new(),
            location: Builder::new(),
        }
    }
}
