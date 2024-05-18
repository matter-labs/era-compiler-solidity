//!
//! Transpile YUL assignments.
//!

use std::iter;

use anyhow::Error;

use crate::easycrypt::translator::Translator;

use crate::easycrypt::syntax::reference::Reference;
use crate::easycrypt::syntax::statement::Statement;
use crate::easycrypt::translator::context::Context;
use crate::easycrypt::translator::expression::context::Context as ExprContext;
use crate::easycrypt::translator::statement::Transformed;
use crate::yul::parser::identifier::Identifier as YulIdentifier;
use crate::yul::parser::statement::assignment::Assignment as YulAssignment;
use crate::yul::parser::statement::expression::Expression as YulExpression;

impl Translator {
    fn bindings_to_references(&self, idents: &[YulIdentifier]) -> Vec<Reference> {
        self.bindings_to_definitions(idents)
            .iter()
            .map(|def| def.reference())
            .collect()
    }

    fn transpile_assignment_aux(
        &mut self,
        bindings: &[YulIdentifier],
        initializer: &YulExpression,
        ctx: &Context,
    ) -> Result<(Context, Transformed), Error> {
        let references = self.bindings_to_references(bindings);

        let (
            new_rhs,
            ExprContext {
                assignments,
                locals,
            },
        ) = self
            .transpile_expression_root(initializer, ctx)?
            .expect_expression_and_get()?;
        let ec_assignment = Statement::EAssignment(references, Box::new(new_rhs));
        let ec_statements = assignments
            .iter()
            .chain(iter::once(&ec_assignment))
            .cloned()
            .collect();
        Ok((
            ctx.add_locals(locals.iter()),
            Transformed::Statements(ec_statements),
        ))
    }
    /// Transpile a YUL assignment.
    pub fn transpile_assignment(
        &mut self,
        assignment: &YulAssignment,
        ctx: &Context,
    ) -> Result<(Context, Transformed), Error> {
        let YulAssignment {
            location: _,
            bindings,
            initializer,
        } = assignment;
        self.transpile_assignment_aux(bindings, initializer, ctx)
    }
}
