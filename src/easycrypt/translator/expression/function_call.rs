//!
//! Transpilation of YUL function calls.
//!

use anyhow::Error;

use crate::easycrypt::syntax::statement::Statement;
use crate::easycrypt::translator::definition_info::kind::YulSpecial;
use crate::easycrypt::translator::expression::Transformed;
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
        is_root: bool,
    ) -> Result<Transformed, Error> {
        match self.transpile_name(ctx, name)? {
            identifier::Translated::Function(target) => {
                let (arguments, ectx) = self.transpile_expression_list(yul_arguments, ctx, ectx)?;
                Ok(Transformed::Expression(
                    Expression::ECall(FunctionCall { target, arguments }),
                    ectx,
                ))
            }

            identifier::Translated::Proc(target) => {
                let (arguments, ectx) = self.transpile_expression_list(yul_arguments, ctx, ectx)?;
                let definition = self.new_tmp_definition_here();
                let mut new_ctx = ectx;

                new_ctx.add_assignment(&definition, ProcCall { target, arguments });
                Ok(Transformed::Expression(
                    Expression::Reference(definition.reference()),
                    new_ctx,
                ))
            }
            identifier::Translated::ProcOrFunction(name) => {
                let (arguments, ectx) = self.transpile_expression_list(yul_arguments, ctx, ectx)?;
                let definition = self.new_tmp_definition_here();
                let mut new_ctx = ectx;

                new_ctx.add_assignment(
                    &definition,
                    ProcCall {
                        target: ProcName::UserDefined(name.clone()),
                        arguments,
                    },
                );
                Ok(Transformed::Expression(
                    Expression::Reference(definition.reference()),
                    new_ctx,
                ))
            }
            identifier::Translated::BinOp(optype) => {
                let (arguments, ectx) = self.transpile_expression_list(yul_arguments, ctx, ectx)?;
                Ok(Transformed::Expression(
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
                Ok(Transformed::Expression(
                    Expression::Unary(optype, Box::new(arguments[0].clone())),
                    ectx,
                ))
            }
            identifier::Translated::Special(special) if is_root => match special {
                YulSpecial::Return => {
                    let (arguments, ectx) =
                        self.transpile_expression_list(yul_arguments, ctx, ectx)?;
                    assert!(ectx.locals.is_empty());
                    assert!(ectx.assignments.is_empty());
                    Ok(Transformed::Statements(
                        vec![Statement::Return(Expression::pack_tuple(&arguments))],
                        ectx,
                        ctx.clone(),
                    ))
                }
                YulSpecial::Stop | YulSpecial::Invalid | YulSpecial::Revert => {
                    //assert!(yul_arguments.is_empty());

                    Ok(Transformed::Statements(
                        vec![Statement::Return(Expression::Tuple(vec![]))],
                        ectx.clone(),
                        ctx.clone(),
                    ))
                }
            },
            identifier::Translated::Special(_) => {
                anyhow::bail!("Unsupported type of YUL function call.")
            }
            identifier::Translated::Variable(_) => anyhow::bail!(
                "Expected a name of function or procedure, got a name of variable instead. {:#?}",
                name
            ),
        }
    }
}
