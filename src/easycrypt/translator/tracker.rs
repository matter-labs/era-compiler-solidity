//!
//! Path tracker for the EasyCrypt to Yul translation.
//!

use crate::easycrypt::syntax::Name;
use crate::yul::path::tracker::PathTracker;
use crate::yul::path::Builder;
use crate::yul::path::Path;

use super::lookup::stack_lookup::Lookup;
use super::lookup::ILookup;

/// Path tracker for the EasyCrypt to Yul translation.
#[derive(Clone, Debug)]
pub struct Tracker {
    lookup: Lookup,
    location: Builder,
}

impl PathTracker for Tracker {
    fn here(&self) -> &Path {
        self.location.here()
    }

    fn leave(&mut self) {
        self.location.leave();
        self.lookup.leave()
    }

    fn enter_block(&mut self) {
        self.location.enter_block();
        self.lookup.enter()
    }

    fn enter_function(&mut self, ident: &str) {
        self.location.enter_function(ident);
        self.lookup.enter()
    }

    fn enter_code(&mut self) {
        self.location.enter_code();
        self.lookup.enter()
    }

    fn enter_object(&mut self, ident: &str) {
        self.location.enter_object(ident);
        self.lookup.enter()
    }

    fn enter_if_cond(&mut self) {
        self.location.enter_if_cond();
        self.lookup.enter()
    }

    fn enter_if_then(&mut self) {
        self.location.enter_if_then();
        self.lookup.enter()
    }

    fn enter_for1(&mut self) {
        self.location.enter_for1();
        self.lookup.enter()
    }

    fn enter_for2(&mut self) {
        self.location.enter_for2();
        self.lookup.enter()
    }

    fn enter_for3(&mut self) {
        self.location.enter_for3();
        self.lookup.enter()
    }
}

impl Tracker {
    pub fn add_var(&mut self, name: &Name) {
        self.lookup.add_var(name, &self.here().clone())
    }

    pub fn add_proc(&mut self, name: &Name) {
        self.lookup.add_proc(name, &self.here().clone())
    }

    pub fn add_fun(&mut self, name: &Name) {
        self.lookup.add_fun(name, &self.here().clone())
    }

    pub fn get(&self, name: &Name) -> Option<super::lookup::Entry> {
        self.lookup.get(name)
    }

    /// Creates a new, empty instance of [`Tracker`]
    pub fn new() -> Self {
        Self {
            lookup: Lookup::new(),
            location: Builder::new(),
        }
    }
}
