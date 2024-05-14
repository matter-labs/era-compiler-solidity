//!
//! Transpilation of YUL blocks of statements.
//!

use anyhow::Error;

use crate::easycrypt::syntax::statement::Statement;
use crate::easycrypt::translator::context::Context;
use crate::easycrypt::translator::statement;
use crate::easycrypt::translator::Translator;
use crate::yul::parser::statement::block::Block as YulBlock;
use crate::yul::path::tracker::PathTracker;

pub struct Transformed {
    pub statements: Vec<Statement>,
}

impl Translator {
    /// Transpile an arbitrary YUL block.
    pub fn transpile_block(
        &mut self,
        block: &YulBlock,
        ctx: &Context,
    ) -> Result<(Context, Transformed), Error> {
        let mut context = ctx.clone();
        let mut result = Transformed {
            statements: Vec::new(),
        };

        self.location_tracker.enter_block();
        for stmt in block.statements.iter() {
            let (ctx, translated) = self.transpile_statement(stmt, &context)?;
            match translated {
                statement::Transformed::Statements(stmts) => {
                    result.statements.extend(stmts);
                }
                statement::Transformed::Function(_) | statement::Transformed::Proc(_) => (),
            };
            context = ctx
        }
        self.location_tracker.leave();
        Ok((context, result))
    }
}
