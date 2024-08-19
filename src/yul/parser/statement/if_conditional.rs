//!
//! The if-conditional statement.
//!

use era_compiler_llvm_context::IContext;

use crate::create_wrapper;
use crate::yul::parser::dialect::llvm::LLVMDialect;
use crate::yul::parser::wrapper::Wrap as _;

use super::expression::WrappedExpression;
create_wrapper!(
    yul_syntax_tools::yul::parser::statement::if_conditional::IfConditional<LLVMDialect>,
    WrappedIfConditional
);

impl<D> era_compiler_llvm_context::EraVMWriteLLVM<D> for WrappedIfConditional
where
    D: era_compiler_llvm_context::Dependency,
{
    fn into_llvm(
        self,
        context: &mut era_compiler_llvm_context::EraVMContext<D>,
    ) -> anyhow::Result<()> {
        let term = self.0;
        let condition = WrappedExpression(term.condition)
            .into_llvm(context)?
            .expect("Always exists")
            .to_llvm()
            .into_int_value();
        let condition = context.builder().build_int_z_extend_or_bit_cast(
            condition,
            context.field_type(),
            "if_condition_extended",
        )?;
        let condition = context.builder().build_int_compare(
            inkwell::IntPredicate::NE,
            condition,
            context.field_const(0),
            "if_condition_compared",
        )?;
        let main_block = context.append_basic_block("if_main");
        let join_block = context.append_basic_block("if_join");
        context.build_conditional_branch(condition, main_block, join_block)?;
        context.set_basic_block(main_block);
        term.block.wrap().into_llvm(context)?;
        context.build_unconditional_branch(join_block)?;
        context.set_basic_block(join_block);

        Ok(())
    }
}

impl<D> era_compiler_llvm_context::EVMWriteLLVM<D> for WrappedIfConditional
where
    D: era_compiler_llvm_context::Dependency,
{
    fn into_llvm(
        self,
        context: &mut era_compiler_llvm_context::EVMContext<D>,
    ) -> anyhow::Result<()> {
        let condition = WrappedExpression(self.0.condition)
            .into_llvm_evm(context)?
            .expect("Always exists")
            .to_llvm()
            .into_int_value();
        let condition = context.builder().build_int_z_extend_or_bit_cast(
            condition,
            context.field_type(),
            "if_condition_extended",
        )?;
        let condition = context.builder().build_int_compare(
            inkwell::IntPredicate::NE,
            condition,
            context.field_const(0),
            "if_condition_compared",
        )?;
        let main_block = context.append_basic_block("if_main");
        let join_block = context.append_basic_block("if_join");
        context.build_conditional_branch(condition, main_block, join_block)?;
        context.set_basic_block(main_block);
        self.0.block.wrap().into_llvm(context)?;
        context.build_unconditional_branch(join_block)?;
        context.set_basic_block(join_block);

        Ok(())
    }
}
