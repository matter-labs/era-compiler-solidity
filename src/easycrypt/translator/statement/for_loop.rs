//!
//! Transpilation of `for` loops in YUL.
//!

use std::iter;

use anyhow::Error;

use crate::easycrypt::syntax::statement::block::Block;
use crate::easycrypt::syntax::statement::while_loop::WhileLoop;
use crate::easycrypt::syntax::statement::Statement;
use crate::easycrypt::translator::block::Transformed as TransformedBlock;
use crate::easycrypt::translator::context::Context;
use crate::easycrypt::translator::expression::context::Context as ExprContext;
use crate::easycrypt::translator::statement::Transformed as TransformedStatement;
use crate::easycrypt::translator::Translator;
use crate::yul::parser::statement::for_loop::ForLoop as YulForLoop;
use crate::yul::path::tracker::PathTracker;

impl Translator {
    /// Transpile a `for` loop.
    /// In the first approximation, `for INIT COND POST BODY` becomes `{ INIT; while (COND) { BODY; POST } }`.
    ///
    /// However, transpiling expressions may result in generating additional
    /// statements if the expression contains a call to a function that becomes
    /// EasyCrypt procedure.
    ///
    /// let then COND, CSTMT be the result of transpiling COND.
    /// Then `for INIT COND POST BODY` becomes `{ INIT; CSTMT; while (COND) { BODY; POST; CSTMT } }`.
    pub fn transpile_for_loop(
        &mut self,
        for_loop: &YulForLoop,
        ctx: &Context,
    ) -> Result<(Context, TransformedStatement), Error> {
        let YulForLoop {
            location: _,
            initializer,
            condition,
            finalizer,
            body,
        } = for_loop;

        self.tracker.leave();
        self.tracker.enter_for1();
        let (
            ctx,
            TransformedBlock {
                statements: transpiled_initializer,
            },
        ) = self.transpile_block(initializer, ctx)?;

        self.tracker.leave();
        self.tracker.enter_for2();
        let (
            transpiled_condition,
            ExprContext {
                assignments,
                locals,
            },
        ) = self.transpile_expression_root(condition, &ctx)?;

        self.tracker.leave();
        self.tracker.enter_for3();
        let (
            ctx,
            TransformedBlock {
                statements: transpiled_finalizer,
            },
        ) = self.transpile_block(finalizer, &ctx)?;
        let (
            new_ctx,
            TransformedBlock {
                statements: transpiled_body,
            },
        ) = self.transpile_block(body, &ctx)?;

        let transpiled_while = WhileLoop {
            condition: transpiled_condition,
            body: Box::from(Statement::Block(Block {
                statements: transpiled_body
                    .iter()
                    .chain(transpiled_finalizer.iter())
                    .chain(assignments.iter())
                    .cloned()
                    .collect(),
            })),
        };

        let transpiled_result = transpiled_initializer
            .iter()
            .chain(assignments.iter())
            .chain(iter::once(&Statement::WhileLoop(transpiled_while)))
            .cloned()
            .collect();

        Ok((
            new_ctx.add_locals(&locals),
            TransformedStatement::Statements(transpiled_result),
        ))
    }
}
