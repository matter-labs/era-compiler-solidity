//!
//! Transpilation context.
//!

use std::iter;

use crate::easycrypt::syntax::{definition::Definition, module::Module};

///
/// Collects the result of the translation and the locals to be emitted in the
/// currently translated procedure.
///
#[derive(Clone, Debug)]
pub struct Context {
    /// Completed definitions so far.
    pub module: Module,
    /// In EasyCrypt, variables are only defined in the procedure scope.
    /// In YUL, variables can be defined in the block scope as well.
    /// Therefore, the translator needs to recursively collect all definitions
    /// of YUL variables from the inner lexical scopes of a YUL function, then
    /// emit all these definitions in the EasyCrypt procedure scope.
    /// This field is used to collect the locals defined in the current function.
    pub locals: Vec<Definition>,
}

impl Context {
    /// Creates a new empty context.
    pub fn new() -> Context {
        Context {
            module: Module::new(None),
            locals: vec![],
        }
    }

    pub fn merge(&mut self, other: &Context) {
        self.module.merge(&other.module);
        self.locals.extend(other.locals.clone());
    }

    pub fn add_locals<'a, I>(&self, definitions: I) -> Self
    where
        I: IntoIterator<Item = &'a Definition>,
    {
        Self {
            module: self.module.clone(),
            locals: self
                .locals
                .iter()
                .cloned()
                .chain(definitions.into_iter().cloned())
                .collect(),
        }
    }

    #[allow(dead_code)]
    pub fn add_local(&self, definition: Definition) -> Self {
        self.add_locals(iter::once(&definition))
    }

    pub fn clear_locals(&self) -> Context {
        Self {
            module: self.module.clone(),
            locals: vec![],
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
