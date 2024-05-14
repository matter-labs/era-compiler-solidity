//!
//! Implementation of [`ILookup`] as a stack of stacks. Stacks are backed by vectors.
//!

use crate::easycrypt::syntax::Name;
use crate::yul::path::Path;

use super::entry::Entry;
use super::kind::Kind;
use super::ILookup;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lookup {
    definitions: Vec<Vec<Entry>>,
}

impl ILookup for Lookup {
    fn add_var(&mut self, name: &Name, path: &Path) {
        self.add(Kind::Variable, name.clone(), path.clone())
    }

    fn add_proc(&mut self, name: &Name, path: &Path) {
        self.add(Kind::Proc, name.clone(), path.clone())
    }

    fn add_fun(&mut self, name: &Name, path: &Path) {
        self.add(Kind::Function, name.clone(), path.clone())
    }

    fn enter(&mut self) {
        self.definitions.push(vec![])
    }

    fn leave(&mut self) {
        let _ = self.definitions.pop();
    }

    fn get(&self, name: &Name) -> Option<Entry> {
        for frame in self.definitions.iter().rev() {
            if let Some(result) = frame.iter().find(|entry| &entry.name == name) {
                return Some(result.clone());
            }
        }
        None
    }
}

impl Lookup {
    pub fn new() -> Self {
        Self {
            definitions: vec![vec![]],
        }
    }

    fn add(&mut self, kind: Kind, name: Name, path: Path) {
        self.definitions
            .last_mut()
            .unwrap()
            .push(Entry { kind, name, path })
    }
}
