//!
//! Transpiler from YUL to EasyCrypt
//!

pub mod block;
pub mod code;
pub mod context;
pub mod expression;
pub mod function;
pub mod identifier;
pub mod object;
pub mod statement;
pub mod tracker;
pub mod r#type;

use crate::util::counter::Counter;
use crate::yul::parser::identifier::Identifier as YulIdentifier;

use self::tracker::Tracker;
use crate::yul::path::tracker::PathTracker;
use crate::yul::path::Path;

use self::context::Context;
use super::syntax::definition::Definition;

use super::syntax::module::definition::TopDefinition;

use super::syntax::r#type::Type;
use super::syntax::reference::Reference;

/// Global state of YUL to EasyCrypt translator
#[derive(Debug)]
pub struct Translator {
    tracker: Tracker,
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
            tracker: Tracker::new(),
            tmp_counter: Counter::new(),
        }
    }

    fn get_module_definition(&self, ctx: &Context, name: &str) -> Option<TopDefinition> {
        let path = self.tracker.get(&name.to_string()).map(|e| e.path);

        let reference = Reference {
            identifier: name.to_owned(),
            location: path,
        };
        ctx.module.definitions.get(&reference).cloned()
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
}
