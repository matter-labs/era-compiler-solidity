//!
//! Expression translation context.
//!

use crate::easycrypt::syntax::definition::Definition;
use crate::easycrypt::syntax::statement::call::ProcCall;
use crate::easycrypt::syntax::statement::Statement;

/// Expression translation context.
#[derive(Clone)]
pub struct Context {
    /// When the root expression is finished translating, assignments will be
    /// prepended to the currently translated statement.
    pub assignments: Vec<Statement>,
    /// When the root expression is finished translating, these definitions
    /// will be appended to the current context; eventually, the corresponding
    /// variable definitions will be emitted in the parent procedure.
    pub locals: Vec<Definition>,
}

impl Context {
    /// Creates a new instance of [`Context`] with an empty state.
    pub fn new() -> Self {
        Self {
            assignments: vec![],
            locals: vec![],
        }
    }

    /// Add a new assignment to the context. When the root expression is
    /// finished translating, all such assignments will be prepended to the
    /// currently translated statement.
    pub fn add_assignment(&mut self, new_definition: &Definition, rhs: ProcCall) {
        self.assignments.push(Statement::PAssignment(
            vec![new_definition.reference()],
            rhs,
        ));
        self.locals.push(new_definition.clone())
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
