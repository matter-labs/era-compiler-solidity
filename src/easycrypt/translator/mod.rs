//!
//! Transpiler from YUL to EasyCrypt
//!

pub mod block;
pub mod code;
pub mod context;
pub mod definition_info;
pub mod expression;
pub mod function;
pub mod identifier;
pub mod object;
pub mod r#type;
pub mod statement;

use crate::util::counter::Counter;
use crate::yul::parser::identifier::Identifier as YulIdentifier;
use crate::yul::path::full_name::FullName;
use crate::yul::path::tracker::symbol_tracker::SymbolTracker;
use crate::yul::path::tracker::PathTracker;
use crate::yul::path::Path;

use super::syntax::definition::Definition;

use super::syntax::r#type::Type;

mod kind {}

/// Global state of YUL to EasyCrypt translator
#[derive(Debug)]
pub struct Translator {
    tracker: SymbolTracker<definition_info::DefinitionInfo>,
    tmp_counter: Counter,
}

impl Default for Translator {
    fn default() -> Self {
        Self::new()
    }
}

impl Translator {
    /// Create an instance of [`Translator`] with an empty state.
    pub fn new() -> Self {
        Self {
            tracker: SymbolTracker::new(),
            tmp_counter: Counter::new(),
        }
    }

    fn new_definition_here(&self, name: &str, typ: Option<Type>) -> Definition {
        Self::new_definition(self.here(), name, typ)
    }

    fn new_tmp_definition_here(&mut self) -> Definition {
        let name = format!("TMP{}", self.tmp_counter.get_value());
        self.tmp_counter.increment();
        Self::new_definition(self.here(), &name, None)
    }

    fn here(&self) -> Path {
        self.tracker.here().clone()
    }

    fn bindings_to_definitions(&self, idents: &[YulIdentifier]) -> Vec<Definition> {
        idents
            .iter()
            .map(|ident| Definition {
                identifier: ident.inner.clone(),
                location: Some(self.here()),
                r#type: ident
                    .r#type
                    .as_ref()
                    .and_then(|t| Self::transpile_type(t).ok()),
            })
            .collect()
    }

    fn new_definition(location: Path, name: &str, r#type: Option<Type>) -> Definition {
        Definition {
            identifier: String::from(name),
            location: Some(location),
            r#type,
        }
    }

    fn create_full_name(&self, identifier: &str) -> FullName {
        FullName::new(identifier.to_string(), self.here())
    }
}
