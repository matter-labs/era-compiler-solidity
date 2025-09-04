//!
//! The if-conditional statement.
//!

use era_compiler_llvm_context::IContext;

use crate::declare_wrapper;
use crate::yul::parser::dialect::era::EraDialect;
use crate::yul::parser::wrapper::Wrap;

use super::expression::Expression;
declare_wrapper!(
    era_yul::yul::parser::statement::if_conditional::IfConditional<EraDialect>,
    IfConditional
);

impl era_compiler_llvm_context::EraVMWriteLLVM for IfConditional {
    fn into_llvm(
        self,
        context: &mut era_compiler_llvm_context::EraVMContext,
    ) -> anyhow::Result<()> {
        let term = self.0;
        let condition = Expression(term.condition)
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
