//!
//! The variable declaration statement.
//!

use era_compiler_llvm_context::IContext;
use inkwell::types::BasicType;
use inkwell::values::BasicValue;

use crate::create_wrapper;
use crate::yul::parser::wrapper::Wrap as _;

use super::expression::WrappedExpression;
create_wrapper!(
    yul_syntax_tools::yul::parser::statement::variable_declaration::VariableDeclaration,
    WrappedVariableDeclaration
);

impl<D> era_compiler_llvm_context::EraVMWriteLLVM<D> for WrappedVariableDeclaration
where
    D: era_compiler_llvm_context::Dependency,
{
    fn into_llvm<'ctx>(
        mut self,
        context: &mut era_compiler_llvm_context::EraVMContext<'ctx, D>,
    ) -> anyhow::Result<()> {
        if self.0.bindings.len() == 1 {
            let identifier = self.0.bindings.remove(0);
            let r#type = identifier
                .r#type
                .unwrap_or_default()
                .wrap()
                .into_llvm(context);
            let pointer = context.build_alloca(r#type, identifier.inner.as_str())?;
            context
                .current_function()
                .borrow_mut()
                .insert_stack_pointer(identifier.inner.clone(), pointer);

            let value = if let Some(expression) = self.0.expression {
                match WrappedExpression(expression).into_llvm(context)? {
                    Some(mut value) => {
                        if let Some(constant) = value.constant.take() {
                            context
                                .current_function()
                                .borrow_mut()
                                .yul_mut()
                                .insert_constant(identifier.inner, constant);
                        }

                        value.to_llvm()
                    }
                    None => r#type.const_zero().as_basic_value_enum(),
                }
            } else {
                r#type.const_zero().as_basic_value_enum()
            };
            context.build_store(pointer, value)?;
            return Ok(());
        }

        for (index, binding) in self.0.bindings.iter().enumerate() {
            let yul_type = binding
                .r#type
                .to_owned()
                .unwrap_or_default()
                .wrap()
                .into_llvm(context);
            let pointer = context.build_alloca(
                yul_type.as_basic_type_enum(),
                format!("binding_{index}_pointer").as_str(),
            )?;
            context.build_store(pointer, yul_type.const_zero())?;
            context
                .current_function()
                .borrow_mut()
                .insert_stack_pointer(binding.inner.to_owned(), pointer);
        }

        let expression = match self.0.expression.take() {
            Some(expression) => expression,
            None => return Ok(()),
        };
        let location = expression.location();
        let expression = match expression.wrap().into_llvm(context)? {
            Some(expression) => expression,
            None => return Ok(()),
        };

        let llvm_type = context.structure_type(
            self.0
                .bindings
                .iter()
                .map(|binding| {
                    binding
                        .r#type
                        .to_owned()
                        .unwrap_or_default()
                        .wrap()
                        .into_llvm(context)
                        .as_basic_type_enum()
                })
                .collect::<Vec<inkwell::types::BasicTypeEnum<'ctx>>>()
                .as_slice(),
        );
        if expression.value.get_type() != llvm_type.as_basic_type_enum() {
            anyhow::bail!(
                "{location} Assignment to {:?} received an invalid number of arguments",
                self.0.bindings
            );
        }
        let pointer = context.build_alloca(llvm_type, "bindings_pointer")?;
        context.build_store(pointer, expression.to_llvm())?;

        for (index, binding) in self.0.bindings.into_iter().enumerate() {
            let pointer = context.build_gep(
                pointer,
                &[
                    context.field_const(0),
                    context
                        .integer_type(era_compiler_common::BIT_LENGTH_X32)
                        .const_int(index as u64, false),
                ],
                binding.r#type.unwrap_or_default().wrap().into_llvm(context),
                format!("binding_{index}_gep_pointer").as_str(),
            )?;

            let value = context.build_load(pointer, format!("binding_{index}_value").as_str())?;
            let pointer = context
                .current_function()
                .borrow_mut()
                .get_stack_pointer(binding.inner.as_str())
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "{} Assignment to an undeclared variable `{}`",
                        binding.location,
                        binding.inner
                    )
                })?;
            context.build_store(pointer, value)?;
        }

        Ok(())
    }
}

impl<D> era_compiler_llvm_context::EVMWriteLLVM<D> for WrappedVariableDeclaration
where
    D: era_compiler_llvm_context::Dependency,
{
    fn into_llvm<'ctx>(
        mut self,
        context: &mut era_compiler_llvm_context::EVMContext<'ctx, D>,
    ) -> anyhow::Result<()> {
        if self.0.bindings.len() == 1 {
            let identifier = self.0.bindings.remove(0);
            let r#type = identifier
                .r#type
                .unwrap_or_default()
                .wrap()
                .into_llvm(context);
            let pointer = context.build_alloca(r#type, identifier.inner.as_str())?;
            context
                .current_function()
                .borrow_mut()
                .insert_stack_pointer(identifier.inner.clone(), pointer);

            let value = if let Some(expression) = self.0.expression {
                match expression.wrap().into_llvm_evm(context)? {
                    Some(value) => value.to_llvm(),
                    None => r#type.const_zero().as_basic_value_enum(),
                }
            } else {
                r#type.const_zero().as_basic_value_enum()
            };
            context.build_store(pointer, value)?;
            return Ok(());
        }

        for (index, binding) in self.0.bindings.iter().enumerate() {
            let yul_type = binding
                .r#type
                .to_owned()
                .unwrap_or_default()
                .wrap()
                .into_llvm(context);
            let pointer = context.build_alloca(
                yul_type.as_basic_type_enum(),
                format!("binding_{index}_pointer").as_str(),
            )?;
            context.build_store(pointer, yul_type.const_zero())?;
            context
                .current_function()
                .borrow_mut()
                .insert_stack_pointer(binding.inner.to_owned(), pointer);
        }

        let expression = match self.0.expression.take() {
            Some(expression) => expression,
            None => return Ok(()),
        };
        let location = expression.location();
        let expression = match expression.wrap().into_llvm_evm(context)? {
            Some(expression) => expression,
            None => return Ok(()),
        };

        let llvm_type = context.structure_type(
            self.0
                .bindings
                .iter()
                .map(|binding| {
                    binding
                        .r#type
                        .to_owned()
                        .unwrap_or_default()
                        .wrap()
                        .into_llvm(context)
                        .as_basic_type_enum()
                })
                .collect::<Vec<inkwell::types::BasicTypeEnum<'ctx>>>()
                .as_slice(),
        );
        if expression.value.get_type() != llvm_type.as_basic_type_enum() {
            anyhow::bail!(
                "{location} Assignment to {:?} received an invalid number of arguments",
                self.0.bindings
            );
        }
        let pointer = context.build_alloca(llvm_type, "bindings_pointer")?;
        context.build_store(pointer, expression.to_llvm())?;

        for (index, binding) in self.0.bindings.into_iter().enumerate() {
            let pointer = context.build_gep(
                pointer,
                &[
                    context.field_const(0),
                    context
                        .integer_type(era_compiler_common::BIT_LENGTH_X32)
                        .const_int(index as u64, false),
                ],
                binding.r#type.unwrap_or_default().wrap().into_llvm(context),
                format!("binding_{index}_gep_pointer").as_str(),
            )?;

            let value = context.build_load(pointer, format!("binding_{index}_value").as_str())?;
            let pointer = context
                .current_function()
                .borrow_mut()
                .get_stack_pointer(binding.inner.as_str())
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "{} Assignment to an undeclared variable `{}`",
                        binding.location,
                        binding.inner
                    )
                })?;
            context.build_store(pointer, value)?;
        }

        Ok(())
    }
}
