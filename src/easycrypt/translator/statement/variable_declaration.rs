//!
//! Transpilation of YUL variable declarations.
//!

use anyhow::Error;

use super::Transformed;
use crate::easycrypt::translator::Translator;
use crate::easycrypt::translator::context::Context;
use crate::yul::parser::statement::assignment::Assignment as YulAssignment;
use crate::yul::parser::statement::variable_declaration::VariableDeclaration;

impl Translator {
    /// Transpiles `var x,y,... = expr` or `var x,y` as follows:
    /// 1. Transform expression `expr`. This may produce additional statements
    /// and new temporary locals when `expr` contains function calls that are
    /// transpiled into procedure calls: each procedure call should be a
    /// distinct statement. All of them should be added to the context `ctx`.
    /// 2. Add `x,y,...` to the list of locals in context `ctx`
    /// 3. Return an assignment, if there was an expression on the right hand side.
    pub fn transpile_variable_declaration(
        &mut self,
        vd: &VariableDeclaration,
        ctx: &Context,
    ) -> Result<(Context, Transformed), Error> {
        let VariableDeclaration {
            location,
            bindings,
            expression,
        } = vd;
        let definitions = self.bindings_to_definitions(bindings);

        let ctx = ctx.add_locals(definitions.iter());
        if let Some(initializer) = expression {
            //self.transpile_assignment_aux(bindings, initializer, &ctx)
            let equivalent_assignment = YulAssignment {
                location: *location,
                initializer: initializer.clone(),
                bindings: bindings.to_vec(),
            };

            self.transpile_assignment(&equivalent_assignment, &ctx)
        } else {
            Ok((ctx, Transformed::Statements(vec![])))
        }
    }
}
