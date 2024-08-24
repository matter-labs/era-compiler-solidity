//!
//! Symbol table where full names are mapped to the values. Getting a definition
//! by name performs lookup in all its parent lexical scopes.
//!

use std::collections::HashMap;
use std::fmt::Debug;

use crate::yul::path::full_name::FullName;

///
/// Symbol table where full names are mapped to the values. Getting a definition
/// by name performs lookup in all its parent lexical scopes.
///
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SymbolTable<T>
where
    T: Clone + Debug + PartialEq + Eq,
{
    contents: HashMap<FullName, T>,
}

impl<T> SymbolTable<T>
where
    T: Clone + Debug + PartialEq + Eq,
{
    ///
    /// Returns a new empty instance of [`SymbolTable`].
    ///
    pub fn new() -> Self {
        Self {
            contents: HashMap::new(),
        }
    }

    ///
    /// Inserts a new name-value pair to the symbol table.
    ///
    pub fn insert(&mut self, name: &FullName, value: &T) {
        self.contents.insert(name.clone(), value.clone());
    }

    ///
    /// Searches for the value by its full name in all its parent lexical scopes.
    ///
    pub fn get<'a>(&'a self, name: &FullName) -> Option<&'a T> {
        for parent in name.path.parents() {
            let full_name = FullName {
                name: name.name.clone(),
                path: parent,
            };
            if let Some(def) = self.contents.get(&full_name) {
                return Some(def);
            }
        }
        None
    }

    ///
    /// Searches for the value by its full name in all its parent lexical scopes. Returns a mutable reference.
    ///
    pub fn get_mut<'a>(&'a mut self, name: &FullName) -> Option<&'a mut T> {
        for parent in name.path.parents() {
            let full_name = FullName {
                name: name.name.clone(),
                path: parent.clone(),
            };
            if self.contents.contains_key(&full_name) {
                return self.contents.get_mut(&full_name);
            }
        }
        None
    }
}
