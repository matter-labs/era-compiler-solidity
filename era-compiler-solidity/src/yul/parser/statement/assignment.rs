//!
//! The assignment expression statement.
//!

use era_compiler_llvm_context::IContext;
use inkwell::types::BasicType;

use crate::declare_wrapper;
use crate::yul::parser::wrapper::Wrap;

declare_wrapper!(
    era_yul::yul::parser::statement::assignment::Assignment,
    Assignment
);

impl era_compiler_llvm_context::EraVMWriteLLVM for Assignment {
    fn into_llvm(
        mut self,
        context: &mut era_compiler_llvm_context::EraVMContext,
    ) -> anyhow::Result<()> {
        let value = match self.0.initializer.wrap().into_llvm(context)? {
            Some(value) => value,
            None => return Ok(()),
        };

        if self.0.bindings.len() == 1 {
            let identifier = self.0.bindings.remove(0);
            let pointer = context
                .current_function()
                .borrow()
                .get_stack_pointer(identifier.inner.as_str())
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "{} Assignment to an undeclared variable `{}`",
                        identifier.location,
                        identifier.inner,
                    )
                })?;
            // The constant annotation attached to a variable must equal that
            // variable's value on every control-flow path. `variable_declaration`
            // caches a literal initializer; an assignment that does not refresh
            // the entry leaves a stale constant behind, and a later read
            // re-attaches it to the new value. Under `--enable-eravm-extensions`
            // that annotation selects the extension intrinsic for a call
            // destination, so a stale entry turns an external call into an
            // unrelated instruction.
            let constant = value.constant.clone();
            context.build_store(pointer, value.to_llvm())?;
            let function = context.current_function();
            let mut function = function.borrow_mut();
            match constant {
                Some(constant) => function
                    .yul_mut()
                    .insert_constant(identifier.inner, constant),
                None => function
                    .yul_mut()
                    .remove_constant(identifier.inner.as_str()),
            }
            return Ok(());
        }

        let llvm_type = value.to_llvm().into_struct_value().get_type();
        let tuple_pointer = context.build_alloca(llvm_type, "assignment_pointer")?;
        context.build_store(tuple_pointer, value.to_llvm())?;

        for (index, binding) in self.0.bindings.into_iter().enumerate() {
            let field_pointer = context.build_gep(
                tuple_pointer,
                &[
                    context.field_const(0),
                    context
                        .integer_type(era_compiler_common::BIT_LENGTH_X32)
                        .const_int(index as u64, false),
                ],
                context.field_type().as_basic_type_enum(),
                format!("assignment_binding_{index}_gep_pointer").as_str(),
            )?;

            let binding_pointer = context
                .current_function()
                .borrow()
                .get_stack_pointer(binding.inner.as_str())
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "{} Assignment to an undeclared variable `{}`",
                        binding.location,
                        binding.inner,
                    )
                })?;
            let value = context.build_load(
                field_pointer,
                format!("assignment_binding_{index}_value").as_str(),
            )?;
            context.build_store(binding_pointer, value)?;
            // A tuple element is loaded from memory and is never a known
            // constant, so any cached entry for this binding is now stale.
            context
                .current_function()
                .borrow_mut()
                .yul_mut()
                .remove_constant(binding.inner.as_str());
        }

        Ok(())
    }
}
