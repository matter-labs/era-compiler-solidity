//!
//! Transpilation of YUL expressions.
//!

pub mod context;
pub mod function_call;
pub mod literal;

use crate::easycrypt::syntax::expression::Expression;
use crate::easycrypt::syntax::reference::Reference;
use crate::easycrypt::syntax::statement::Statement;
use crate::easycrypt::translator::context::Context;
use crate::easycrypt::translator::expression::context::Context as ExprContext;
use crate::yul::parser::statement::expression::function_call::FunctionCall as YulFunctionCall;
use crate::yul::parser::statement::expression::Expression as YulExpression;
use crate::Translator;
use anyhow::Error;

#[derive(Debug)]
pub enum Transformed {
    Expression(Expression, ExprContext),
    Statements(Vec<Statement>, ExprContext, Context),
}

impl Transformed {
    pub fn expect_expression_and_get(self) -> Result<(Expression, ExprContext), Error> {
        if let Self::Expression(expr, ectx) = self {
            Ok((expr, ectx))
        } else {
            anyhow::bail!(
                format!("{} \n {:#?}", Self::MSG_EXPECTED_EXPRESSION_RESULT, &self).to_string()
            )
        }
    }

    pub const MSG_EXPECTED_EXPRESSION_RESULT : &'static str = "Malformed YUL: all expressions in an expression list are supposed to be transpiled into expressions. Expressions like `return` or `stop` are transpiled into statements.";
}

impl Translator {
    /// Transpile multiple YUL expressions accumulating the context.
    pub fn transpile_expression_list(
        &mut self,
        list: &[YulExpression],
        ctx: &Context,
        ectx: &ExprContext,
    ) -> Result<(Vec<Expression>, ExprContext), Error> {
        let mut ectx: ExprContext = ectx.clone();
        let mut result: Vec<Expression> = Vec::new();

        for expr in list {
            if let Transformed::Expression(e, new_ectx) =
                self.transpile_expression(expr, ctx, &ectx, false)?
            {
                ectx = new_ectx;
                result.push(e);
            } else {
                anyhow::bail!(Transformed::MSG_EXPECTED_EXPRESSION_RESULT)
            }
        }
        Ok((result, ectx))
    }

    /// Transpile an arbitrary YUL expression.
    fn transpile_expression(
        &mut self,
        expr: &YulExpression,
        ctx: &Context,
        ectx: &ExprContext,
        is_root: bool,
    ) -> Result<Transformed, Error> {
        match expr {
            YulExpression::FunctionCall(YulFunctionCall {
                location: _,
                name,
                arguments,
            }) => self.transpile_function_call(name, arguments, ctx, ectx, is_root),

            YulExpression::Identifier(ident) =>
            // FIXME: visit identifier
            {
                Ok(Transformed::Expression(
                    Expression::Reference(Reference {
                        identifier: ident.inner.clone(),
                        location: Some(self.here()),
                    }),
                    ectx.clone(),
                ))
            }
            YulExpression::Literal(lit) => Ok(Transformed::Expression(
                Expression::Literal(Self::transpile_literal(lit)?),
                ectx.clone(),
            )),
        }
    }

    /// Transpile a YUL expression that is not a subexpression of any other expression.
    pub fn transpile_expression_root(
        &mut self,
        expr: &YulExpression,
        ctx: &Context,
    ) -> Result<Transformed, Error> {
        self.transpile_expression(expr, ctx, &ExprContext::new(), true)
    }
}
