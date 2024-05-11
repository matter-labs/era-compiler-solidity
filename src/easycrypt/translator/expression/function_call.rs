use anyhow::Error;

use crate::Translator;

use crate::easycrypt::syntax::expression::call::FunctionCall;
use crate::easycrypt::syntax::expression::Expression;
use crate::easycrypt::syntax::proc::name::ProcName;
use crate::easycrypt::syntax::statement::call::ProcCall;
use crate::easycrypt::translator::identifier;
use crate::yul::parser::statement::expression::function_call::name::Name as YulName;
use crate::yul::parser::statement::expression::Expression as YulExpression;

use crate::easycrypt::translator::context::Context;
use crate::easycrypt::translator::expression::context::Context as ExprContext;

impl Translator {
    /// Transpile a function call in YUL into either EasyCrypt procedure or function.
    pub fn transpile_function_call(
        &mut self,
        name: &YulName,
        yul_arguments: &[YulExpression],
        ctx: &Context,
        ectx: &ExprContext,
    ) -> Result<(Expression, ExprContext), Error> {
        match self.transpile_name(ctx, name) {
            identifier::Translated::Function(target) => {
                let (arguments, ectx) = self.transpile_expression_list(yul_arguments, ctx, ectx)?;
                Ok((Expression::ECall(FunctionCall { target, arguments }), ectx))
            }

            identifier::Translated::Proc(target) => {
                let (arguments, ctx) = self.transpile_expression_list(yul_arguments, ctx, ectx)?;
                let definition = self.new_tmp_definition_here();
                let mut new_ctx = ctx;

                new_ctx.add_assignment(&definition, ProcCall { target, arguments });
                Ok((Expression::Reference(definition.reference()), new_ctx))
            }
            identifier::Translated::ProcOrFunction(name) => {
                let (arguments, ctx) = self.transpile_expression_list(yul_arguments, ctx, ectx)?;
                let definition = self.new_tmp_definition_here();
                let mut new_ctx = ctx;

                new_ctx.add_assignment(
                    &definition,
                    ProcCall {
                        target: ProcName::UserDefined(name.clone()),
                        arguments,
                    },
                );
                Ok((Expression::Reference(definition.reference()), new_ctx))
            }
            identifier::Translated::BinOp(optype) => {
                let (arguments, ectx) = self.transpile_expression_list(yul_arguments, ctx, ectx)?;
                Ok((
                    Expression::Binary(
                        optype,
                        Box::new(arguments[0].clone()),
                        Box::new(arguments[1].clone()),
                    ),
                    ectx,
                ))
            }
            identifier::Translated::UnOp(optype) => {
                let (arguments, ectx) = self.transpile_expression_list(yul_arguments, ctx, ectx)?;
                Ok((
                    Expression::Unary(optype, Box::new(arguments[0].clone())),
                    ectx,
                ))
            }
            identifier::Translated::Special(_) => todo!(),
        }
    }
}
