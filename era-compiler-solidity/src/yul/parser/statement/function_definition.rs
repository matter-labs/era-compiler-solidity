//!
//! The function definition statement.
//!

use era_compiler_llvm_context::IContext;
use inkwell::types::BasicType;

use crate::declare_wrapper;
use crate::yul::parser::dialect::era::EraDialect;
use crate::yul::parser::wrapper::Wrap;

declare_wrapper!(
    era_yul::yul::parser::statement::function_definition::FunctionDefinition<EraDialect>,
    FunctionDefinition
);

impl<D> era_compiler_llvm_context::EraVMWriteLLVM<D> for FunctionDefinition
where
    D: era_compiler_llvm_context::Dependency + Clone,
{
    fn declare(
        &mut self,
        context: &mut era_compiler_llvm_context::EraVMContext<D>,
    ) -> anyhow::Result<()> {
        let argument_types: Vec<_> = self
            .0
            .arguments
            .iter()
            .map(|argument| {
                let yul_type = argument.r#type.to_owned().unwrap_or_default();
                yul_type.wrap().into_llvm(context).as_basic_type_enum()
            })
            .collect();

        let function_type = context.function_type(
            argument_types,
            self.0.result.len(),
            self.0
                .identifier
                .starts_with(era_compiler_llvm_context::EraVMFunction::ZKSYNC_NEAR_CALL_ABI_PREFIX),
        );

        let function = context.add_function(
            self.0.identifier.as_str(),
            function_type,
            self.0.result.len(),
            Some(inkwell::module::Linkage::Private),
        )?;
        era_compiler_llvm_context::EraVMFunction::set_attributes(
            context.llvm(),
            function.borrow().declaration().value,
            self.0
                .attributes
                .clone()
                .into_iter()
                .map(|attribute| (attribute, None))
                .collect(),
            true,
        );
        function
            .borrow_mut()
            .set_yul_data(era_compiler_llvm_context::EraVMFunctionYulData::default());

        Ok(())
    }

    fn into_llvm(
        mut self,
        context: &mut era_compiler_llvm_context::EraVMContext<D>,
    ) -> anyhow::Result<()> {
        context.set_current_function(self.0.identifier.as_str())?;
        let r#return = context.current_function().borrow().r#return();

        context.set_basic_block(context.current_function().borrow().entry_block());
        match r#return {
            era_compiler_llvm_context::FunctionReturn::None => {}
            era_compiler_llvm_context::FunctionReturn::Primitive { pointer } => {
                let identifier = self.0.result.pop().expect("Always exists");
                let r#type = identifier.r#type.unwrap_or_default();
                context.build_store(pointer, r#type.wrap().into_llvm(context).const_zero())?;
                context
                    .current_function()
                    .borrow_mut()
                    .insert_stack_pointer(identifier.inner, pointer);
            }
            era_compiler_llvm_context::FunctionReturn::Compound { pointer, .. } => {
                for (index, identifier) in self.0.result.into_iter().enumerate() {
                    let r#type = identifier
                        .r#type
                        .unwrap_or_default()
                        .wrap()
                        .into_llvm(context);
                    let pointer = context.build_gep(
                        pointer,
                        &[
                            context.field_const(0),
                            context
                                .integer_type(era_compiler_common::BIT_LENGTH_X32)
                                .const_int(index as u64, false),
                        ],
                        context.field_type(),
                        format!("return_{index}_gep_pointer").as_str(),
                    )?;
                    context.build_store(pointer, r#type.const_zero())?;
                    context
                        .current_function()
                        .borrow_mut()
                        .insert_stack_pointer(identifier.inner.clone(), pointer);
                }
            }
        };

        let argument_types: Vec<_> = self
            .0
            .arguments
            .iter()
            .map(|argument| {
                let yul_type = argument.r#type.to_owned().unwrap_or_default();
                yul_type.wrap().into_llvm(context)
            })
            .collect();
        for (mut index, argument) in self.0.arguments.iter().enumerate() {
            let pointer = context.build_alloca(argument_types[index], argument.inner.as_str())?;
            context
                .current_function()
                .borrow_mut()
                .insert_stack_pointer(argument.inner.clone(), pointer);
            if self
                .0
                .identifier
                .starts_with(era_compiler_llvm_context::EraVMFunction::ZKSYNC_NEAR_CALL_ABI_PREFIX)
                && matches!(
                    context.current_function().borrow().r#return(),
                    era_compiler_llvm_context::FunctionReturn::Compound { .. }
                )
                && context.are_eravm_extensions_enabled()
            {
                index += 1;
            }
            context.build_store(
                pointer,
                context.current_function().borrow().get_nth_param(index),
            )?;
        }

        self.0.body.wrap().into_llvm(context)?;
        match context
            .basic_block()
            .get_last_instruction()
            .map(|instruction| instruction.get_opcode())
        {
            Some(inkwell::values::InstructionOpcode::Br) => {}
            Some(inkwell::values::InstructionOpcode::Switch) => {}
            _ => context
                .build_unconditional_branch(context.current_function().borrow().return_block())?,
        }

        context.set_basic_block(context.current_function().borrow().return_block());
        match context.current_function().borrow().r#return() {
            era_compiler_llvm_context::FunctionReturn::None => {
                context.build_return(None)?;
            }
            era_compiler_llvm_context::FunctionReturn::Primitive { pointer } => {
                let return_value = context.build_load(pointer, "return_value")?;
                context.build_return(Some(&return_value))?;
            }
            era_compiler_llvm_context::FunctionReturn::Compound { pointer, .. }
                if context.current_function().borrow().name().starts_with(
                    era_compiler_llvm_context::EraVMFunction::ZKSYNC_NEAR_CALL_ABI_PREFIX,
                ) =>
            {
                context.build_return(Some(&pointer.value))?;
            }
            era_compiler_llvm_context::FunctionReturn::Compound { pointer, .. } => {
                let return_value = context.build_load(pointer, "return_value")?;
                context.build_return(Some(&return_value))?;
            }
        }

        Ok(())
    }
}

impl<D> era_compiler_llvm_context::EVMWriteLLVM<D> for FunctionDefinition
where
    D: era_compiler_llvm_context::Dependency + Clone,
{
    fn declare(
        &mut self,
        context: &mut era_compiler_llvm_context::EVMContext<D>,
    ) -> anyhow::Result<()> {
        let argument_types: Vec<_> = self
            .0
            .arguments
            .iter()
            .map(|argument| {
                let yul_type = argument.r#type.to_owned().unwrap_or_default();
                yul_type.wrap().into_llvm(context).as_basic_type_enum()
            })
            .collect();

        let function_type = context.function_type(argument_types, self.0.result.len());

        context.add_function(
            self.0.identifier.as_str(),
            function_type,
            self.0.result.len(),
            Some(inkwell::module::Linkage::Private),
        )?;

        Ok(())
    }

    fn into_llvm(
        mut self,
        context: &mut era_compiler_llvm_context::EVMContext<D>,
    ) -> anyhow::Result<()> {
        context.set_current_function(self.0.identifier.as_str())?;
        let r#return = context.current_function().borrow().r#return();

        context.set_basic_block(context.current_function().borrow().entry_block());
        match r#return {
            era_compiler_llvm_context::FunctionReturn::None => {}
            era_compiler_llvm_context::FunctionReturn::Primitive { pointer } => {
                let identifier = self.0.result.pop().expect("Always exists");
                let r#type = identifier.r#type.unwrap_or_default();
                context.build_store(pointer, r#type.wrap().into_llvm(context).const_zero())?;
                context
                    .current_function()
                    .borrow_mut()
                    .insert_stack_pointer(identifier.inner, pointer);
            }
            era_compiler_llvm_context::FunctionReturn::Compound { pointer, .. } => {
                for (index, identifier) in self.0.result.into_iter().enumerate() {
                    let r#type = identifier
                        .r#type
                        .unwrap_or_default()
                        .wrap()
                        .into_llvm(context);
                    let pointer = context.build_gep(
                        pointer,
                        &[
                            context.field_const(0),
                            context
                                .integer_type(era_compiler_common::BIT_LENGTH_X32)
                                .const_int(index as u64, false),
                        ],
                        context.field_type(),
                        format!("return_{index}_gep_pointer").as_str(),
                    )?;
                    context.build_store(pointer, r#type.const_zero())?;
                    context
                        .current_function()
                        .borrow_mut()
                        .insert_stack_pointer(identifier.inner.clone(), pointer);
                }
            }
        };

        let argument_types: Vec<_> = self
            .0
            .arguments
            .iter()
            .map(|argument| {
                let yul_type = argument.r#type.to_owned().unwrap_or_default();
                yul_type.wrap().into_llvm(context)
            })
            .collect();
        for (index, argument) in self.0.arguments.iter().enumerate() {
            let pointer = context.build_alloca(argument_types[index], argument.inner.as_str())?;
            context
                .current_function()
                .borrow_mut()
                .insert_stack_pointer(argument.inner.clone(), pointer);
            context.build_store(
                pointer,
                context.current_function().borrow().get_nth_param(index),
            )?;
        }

        self.0.body.wrap().into_llvm(context)?;
        match context
            .basic_block()
            .get_last_instruction()
            .map(|instruction| instruction.get_opcode())
        {
            Some(inkwell::values::InstructionOpcode::Br) => {}
            Some(inkwell::values::InstructionOpcode::Switch) => {}
            _ => context
                .build_unconditional_branch(context.current_function().borrow().return_block())?,
        }

        context.set_basic_block(context.current_function().borrow().return_block());
        match context.current_function().borrow().r#return() {
            era_compiler_llvm_context::FunctionReturn::None => {
                context.build_return(None)?;
            }
            era_compiler_llvm_context::FunctionReturn::Primitive { pointer } => {
                let return_value = context.build_load(pointer, "return_value")?;
                context.build_return(Some(&return_value))?;
            }
            era_compiler_llvm_context::FunctionReturn::Compound { pointer, .. } => {
                let return_value = context.build_load(pointer, "return_value")?;
                context.build_return(Some(&return_value))?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
///
/// This module contains only dialect-specific tests.
///
mod tests {
    use std::collections::BTreeSet;

    use crate::yul::parser::dialect::era::EraDialect;
    use era_yul::yul::lexer::token::location::Location;
    use era_yul::yul::lexer::Lexer;
    use era_yul::yul::parser::error::Error;
    use era_yul::yul::parser::statement::object::Object;

    #[test]
    fn error_invalid_number_of_arguments_near_call_abi() {
        let input = r#"
object "Test" {
    code {
        {
            return(0, 0)
        }
    }
    object "Test_deployed" {
        code {
            {
                return(0, 0)
            }

            function ZKSYNC_NEAR_CALL_test() -> result {
                result := 42
            }
        }
    }
}
    "#;

        let mut lexer = Lexer::new(input.to_owned());
        let result = Object::<EraDialect>::parse(&mut lexer, None);
        assert_eq!(
            result,
            Err(Error::InvalidNumberOfArguments {
                location: Location::new(14, 22),
                identifier: "ZKSYNC_NEAR_CALL_test".to_owned(),
                expected: 1,
                found: 0,
            }
            .into())
        );
    }

    #[test]
    fn error_invalid_number_of_arguments_near_call_abi_catch() {
        let input = r#"
object "Test" {
    code {
        {
            return(0, 0)
        }
    }
    object "Test_deployed" {
        code {
            {
                return(0, 0)
            }

            function ZKSYNC_CATCH_NEAR_CALL(length) {
                revert(0, length)
            }
        }
    }
}
    "#;

        let mut lexer = Lexer::new(input.to_owned());
        let result = Object::<EraDialect>::parse(&mut lexer, None);
        assert_eq!(
            result,
            Err(Error::InvalidNumberOfArguments {
                location: Location::new(14, 22),
                identifier: "ZKSYNC_CATCH_NEAR_CALL".to_owned(),
                expected: 0,
                found: 1,
            }
            .into())
        );
    }

    #[test]
    fn error_invalid_attributes_single() {
        let input = r#"
object "Test" {
    code {
        {
            return(0, 0)
        }
    }
    object "Test_deployed" {
        code {
            {
                return(0, 0)
            }

            function test_$llvm_UnknownAttribute_llvm$_test() -> result {
                result := 42
            }
        }
    }
}
    "#;
        let mut invalid_attributes = BTreeSet::new();
        invalid_attributes.insert("UnknownAttribute".to_owned());

        let mut lexer = Lexer::new(input.to_owned());
        let result = Object::<EraDialect>::parse(&mut lexer, None);
        assert_eq!(
            result,
            Err(Error::InvalidAttributes {
                location: Location::new(14, 22),
                values: invalid_attributes,
            }
            .into())
        );
    }

    #[test]
    fn error_invalid_attributes_multiple_repeated() {
        let input = r#"
object "Test" {
    code {
        {
            return(0, 0)
        }
    }
    object "Test_deployed" {
        code {
            {
                return(0, 0)
            }

            function test_$llvm_UnknownAttribute1_UnknownAttribute1_UnknownAttribute2_llvm$_test() -> result {
                result := 42
            }
        }
    }
}
    "#;
        let mut invalid_attributes = BTreeSet::new();
        invalid_attributes.insert("UnknownAttribute1".to_owned());
        invalid_attributes.insert("UnknownAttribute2".to_owned());

        let mut lexer = Lexer::new(input.to_owned());
        let result = Object::<EraDialect>::parse(&mut lexer, None);
        assert_eq!(
            result,
            Err(Error::InvalidAttributes {
                location: Location::new(14, 22),
                values: invalid_attributes,
            }
            .into())
        );
    }
}
