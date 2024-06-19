//!
//! EasyCrypt AST node containing a definition of a module.
//!

pub mod definition;

use std::collections::HashMap;

use crate::easycrypt::syntax::module::definition::TopDefinition;
use crate::easycrypt::syntax::reference::Reference;
use crate::easycrypt::syntax::Name;

/// EasyCrypt AST node containing a definition of a module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    /// Name of the module, derived from the name of YUL object.
    pub name: Option<Name>,
    /// Definitions belonging to the module, including functions and procedures.
    pub definitions: HashMap<Reference, TopDefinition>,
}

impl Module {
    /// Creates a new empty instance of [`Module`].
    pub fn new(name: Option<Name>) -> Self {
        Self {
            definitions: HashMap::new(),
            name,
        }
    }

    /// Merge this module with another, nameless module.
    pub fn merge(&mut self, other: &Self) {
        if other.name.is_none() {
            self.definitions.extend(other.definitions.clone())
        } else {
            panic!("Trying to merge a named module with another named module, but only merging with a nameless module is allowed.")
        }
    }

    pub fn add_def(&mut self, module_def: TopDefinition) {
        self.definitions.insert(module_def.reference(), module_def);
    }
}
