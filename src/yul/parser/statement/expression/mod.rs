//!
//! The expression statement.
//!

pub mod function_call;
pub mod literal;

use crate::create_wrapper;
use era_compiler_llvm_context::IContext;

use crate::yul::parser::wrapper::Wrap as _;
use yul_syntax_tools::yul::parser::statement::expression::Expression;

create_wrapper!(Expression, WrappedExpression);

impl WrappedExpression {
    ///
    /// Converts the expression into an LLVM value.
    ///
    pub fn into_llvm<'ctx, D>(
        self,
        context: &mut era_compiler_llvm_context::EraVMContext<'ctx, D>,
    ) -> anyhow::Result<Option<era_compiler_llvm_context::Value<'ctx>>>
    where
        D: era_compiler_llvm_context::Dependency,
    {
        match self.0 {
            Expression::Literal(literal) => literal
                .clone()
                .wrap()
                .into_llvm(context)
                .map_err(|error| {
                    anyhow::anyhow!(
                        "{} Invalid literal `{}`: {}",
                        literal.location,
                        literal.inner.to_string(),
                        error
                    )
                })
                .map(Some),
            Expression::Identifier(identifier) => {
                let pointer = context
                    .current_function()
                    .borrow()
                    .get_stack_pointer(identifier.inner.as_str())
                    .ok_or_else(|| {
                        anyhow::anyhow!(
                            "{} Undeclared variable `{}`",
                            identifier.location,
                            identifier.inner,
                        )
                    })?;

                let constant = context
                    .current_function()
                    .borrow()
                    .yul()
                    .get_constant(identifier.inner.as_str());

                let value = context.build_load(pointer, identifier.inner.as_str())?;

                match constant {
                    Some(constant) => Ok(Some(
                        era_compiler_llvm_context::Value::new_with_constant(value, constant),
                    )),
                    None => Ok(Some(value.into())),
                }
            }
            Expression::FunctionCall(call) => Ok(call
                .wrap()
                .into_llvm(context)?
                .map(era_compiler_llvm_context::Value::new)),
        }
    }

    ///
    /// Converts the expression into an LLVM value.
    ///
    /// TODO: trait
    ///
    pub fn into_llvm_evm<'ctx, D>(
        self,
        context: &mut era_compiler_llvm_context::EVMContext<'ctx, D>,
    ) -> anyhow::Result<Option<era_compiler_llvm_context::Value<'ctx>>>
    where
        D: era_compiler_llvm_context::Dependency,
    {
        match self.0 {
            Expression::Literal(literal) => literal
                .clone()
                .wrap()
                .into_llvm(context)
                .map_err(|error| {
                    anyhow::anyhow!(
                        "{} Invalid literal `{}`: {}",
                        literal.location,
                        literal.inner.to_string(),
                        error
                    )
                })
                .map(Some),
            Expression::Identifier(identifier) => {
                let pointer = context
                    .current_function()
                    .borrow()
                    .get_stack_pointer(identifier.inner.as_str())
                    .ok_or_else(|| {
                        anyhow::anyhow!(
                            "{} Undeclared variable `{}`",
                            identifier.location,
                            identifier.inner,
                        )
                    })?;

                let value = context.build_load(pointer, identifier.inner.as_str())?;
                Ok(Some(value.into()))
            }
            Expression::FunctionCall(call) => Ok(call
                .wrap()
                .into_llvm_evm(context)?
                .map(era_compiler_llvm_context::Value::new)),
        }
    }
}
