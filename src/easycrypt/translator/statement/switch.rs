//!
//! Transpile a "switch" statement in YUL.
//!

use std::iter;

use anyhow::Error;

use crate::easycrypt::syntax::expression::binary::BinaryOpType;
use crate::easycrypt::syntax::expression::Expression;
use crate::easycrypt::syntax::statement::block::Block;
use crate::easycrypt::syntax::statement::if_conditional::IfConditional;
use crate::easycrypt::syntax::statement::Statement;
use crate::easycrypt::translator::block::Transformed as TransformedBlock;
use crate::easycrypt::translator::context::Context;
use crate::easycrypt::translator::expression::context::Context as ExprContext;
use crate::easycrypt::translator::statement::Transformed as TransformedStatement;
use crate::easycrypt::translator::Translator;
use crate::yul::parser::statement::switch::Switch as YulSwitch;

impl Translator {
    /// Transpile a switch statement in YUL.
    ///
    /// switch expr case val1 {} case val2 {}
    /// switch expr case val1 {} case val2 {} default {}
    ///
    /// This should become:
    /// let tmp = expr;
    pub fn transpile_switch(
        &mut self,
        switch: &YulSwitch,
        ctx: &Context,
    ) -> Result<(Context, TransformedStatement), Error> {
        let YulSwitch {
            location: _,
            expression,
            cases,
            default,
        } = switch;

        let (
            transpiled_expression,
            ExprContext {
                assignments,
                locals,
            },
        ) = self
            .transpile_expression_root(expression, ctx)?
            .expect_expression_and_get()?;
        let mut ctx = ctx.add_locals(locals.iter());

        let tmp_def = self.new_tmp_definition_here();
        ctx = ctx.add_local(tmp_def.clone());
        let tmp_ref = tmp_def.reference();
        let initial_assignment =
            Statement::EAssignment(vec![tmp_ref.clone()], Box::from(transpiled_expression));

        let mut result = assignments
            .iter()
            .chain(iter::once(&initial_assignment))
            .cloned()
            .collect::<Vec<_>>();

        for (index, yul_case) in cases.iter().enumerate() {
            let transpiled_case_literal = Self::transpile_literal(&yul_case.literal)?;

            let (new_ctx, TransformedBlock { statements }) =
                self.transpile_block(&yul_case.block, &ctx)?;

            let if_stmt = Statement::If(IfConditional {
                condition: Expression::Binary(
                    BinaryOpType::Eq,
                    Box::from(Expression::Reference(tmp_ref.clone())),
                    Box::from(Expression::Literal(transpiled_case_literal)),
                ),
                yes: Box::from(Statement::Block(Block { statements })),
                no: None,
            });

            ctx = new_ctx.clone();
            if index == 0 {
                result.push(if_stmt)
            } else if let Statement::If(ref mut last_if) = result.last_mut().unwrap() {
                last_if.no = Some(Box::from(Statement::Block(Block {
                    statements: vec![if_stmt],
                })))
            }
        }

        if let Some(block) = default {
            let (new_ctx, TransformedBlock { statements }) = self.transpile_block(block, &ctx)?;
            if cases.is_empty() {
                result.extend(statements)
            } else if let Statement::If(ref mut last_if) = result.last_mut().unwrap() {
                last_if.no = Some(Box::from(Statement::Block(Block { statements })))
            }

            ctx = new_ctx
        }

        Ok((ctx, TransformedStatement::Statements(result)))
    }
}
