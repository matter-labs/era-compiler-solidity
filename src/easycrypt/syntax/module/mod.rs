use std::collections::HashMap;

use self::definition::TopDefinition;

use super::{reference::Reference, Name};

pub mod definition;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub name: Option<Name>,
    pub definitions: HashMap<Reference, TopDefinition>,
}

impl Module {
    pub fn new(name: Option<Name>) -> Self {
        Self {
            definitions: HashMap::new(),
            name,
        }
    }

    #[allow(dead_code)]
    pub fn merge(&mut self, other: &Self) {
        if other.name.is_none() {
            self.definitions.extend(other.definitions.clone())
        } else {
            panic!("Trying to merge named modules")
        }
    }

    #[allow(dead_code)]
    pub fn add_def(&mut self, module_def: TopDefinition) {
        self.definitions.insert(module_def.reference(), module_def);
    }
}
