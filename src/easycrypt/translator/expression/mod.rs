pub mod context;
pub mod function_call;
pub mod literal;

use crate::easycrypt::syntax::expression::Expression;
use crate::easycrypt::syntax::reference::Reference;
use crate::easycrypt::translator::context::Context;
use crate::easycrypt::translator::expression::context::Context as ExprContext;
use crate::yul::parser::statement::expression::function_call::FunctionCall as YulFunctionCall;
use crate::yul::parser::statement::expression::Expression as YulExpression;
use crate::Translator;
use anyhow::Error;

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
            let (e, new_ectx) = self.transpile_expression(expr, ctx, &ectx)?;
            ectx = new_ectx;
            result.push(e);
        }
        Ok((result, ectx))
    }

    /// Transpile an arbitrary YUL expression.
    fn transpile_expression(
        &mut self,
        expr: &YulExpression,
        ctx: &Context,
        ectx: &ExprContext,
    ) -> Result<(Expression, ExprContext), Error> {
        match expr {
            YulExpression::FunctionCall(YulFunctionCall {
                location: _,
                name,
                arguments,
            }) => self.transpile_function_call(name, arguments, ctx, ectx),

            YulExpression::Identifier(ident) =>
            // FIXME: visit identifier
            {
                Ok((
                    Expression::Reference(Reference {
                        identifier: ident.inner.clone(),
                        location: Some(self.here()),
                    }),
                    ectx.clone(),
                ))
            }
            YulExpression::Literal(lit) => Ok((
                Expression::Literal(Self::transpile_literal(lit)),
                ectx.clone(),
            )),
        }
    }

    /// Transpile a YUL expression that is not a subexpression of any other expression.
    pub fn transpile_expression_root(
        &mut self,
        expr: &YulExpression,
        ctx: &Context,
    ) -> Result<(Expression, ExprContext), Error> {
        self.transpile_expression(expr, ctx, &ExprContext::new())
    }
}
