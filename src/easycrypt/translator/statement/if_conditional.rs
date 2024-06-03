//!
//! Transpile an "if" conditional statement in YUL.
//!

use std::iter;

use anyhow::Error;

use crate::Translator;

use crate::easycrypt::syntax::statement::block::Block;
use crate::easycrypt::syntax::statement::if_conditional::IfConditional;
use crate::easycrypt::syntax::statement::Statement;
use crate::easycrypt::translator::block::Transformed as TransformedBlock;
use crate::easycrypt::translator::context::Context;
use crate::easycrypt::translator::expression::context::Context as ExprContext;
use crate::easycrypt::translator::statement::Transformed;
use crate::yul::parser::statement::if_conditional::IfConditional as YulIfConditional;
use crate::yul::path::tracker::PathTracker;

impl Translator {
    /// Transpile an "if" conditional statement.
    pub fn transpile_if(
        &mut self,
        conditional: &YulIfConditional,
        ctx: &Context,
    ) -> Result<(Context, Transformed), Error> {
        let YulIfConditional {
            location: _,
            condition,
            block,
        } = conditional;
        self.tracker.enter_if_cond();
        let (
            transpiled_condition,
            ExprContext {
                assignments,
                locals,
            },
        ) = self
            .transpile_expression_root(condition, ctx)?
            .expect_expression_and_get()?;
        let ctx = ctx.add_locals(&locals);

        self.tracker.leave();
        self.tracker.enter_if_then();

        let (ctx, TransformedBlock { statements }) = self.transpile_block(block, &ctx)?;

        let transpiled_conditional = IfConditional {
            condition: transpiled_condition,
            yes: Box::from(Statement::Block(Block { statements })),
            no: None,
        };
        self.tracker.leave();

        let result = Transformed::Statements(
            assignments
                .iter()
                .chain(iter::once(&Statement::If(transpiled_conditional)))
                .cloned()
                .collect(),
        );
        Ok((ctx, result))
    }
}