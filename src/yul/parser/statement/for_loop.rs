//!
//! The for-loop statement.
//!

use era_compiler_llvm_context::IContext;

use crate::{
    create_wrapper,
    yul::parser::{dialect::llvm::LLVMDialect, wrapper::Wrap as _},
};

use super::expression::WrappedExpression;

create_wrapper!(
    yul_syntax_tools::yul::parser::statement::for_loop::ForLoop<LLVMDialect>,
    WrappedForLoop
);

impl<D> era_compiler_llvm_context::EraVMWriteLLVM<D> for WrappedForLoop
where
    D: era_compiler_llvm_context::Dependency,
{
    fn into_llvm(
        self,
        context: &mut era_compiler_llvm_context::EraVMContext<D>,
    ) -> anyhow::Result<()> {
        let term = self.0;
        term.initializer.wrap().into_llvm(context)?;

        let condition_block = context.append_basic_block("for_condition");
        let body_block = context.append_basic_block("for_body");
        let increment_block = context.append_basic_block("for_increment");
        let join_block = context.append_basic_block("for_join");

        context.build_unconditional_branch(condition_block)?;
        context.set_basic_block(condition_block);
        let condition = WrappedExpression(term.condition)
            .into_llvm(context)?
            .expect("Always exists")
            .to_llvm()
            .into_int_value();
        let condition = context.builder().build_int_z_extend_or_bit_cast(
            condition,
            context.field_type(),
            "for_condition_extended",
        )?;
        let condition = context.builder().build_int_compare(
            inkwell::IntPredicate::NE,
            condition,
            context.field_const(0),
            "for_condition_compared",
        )?;
        context.build_conditional_branch(condition, body_block, join_block)?;

        context.push_loop(body_block, increment_block, join_block);

        context.set_basic_block(body_block);
        term.body.wrap().into_llvm(context)?;
        context.build_unconditional_branch(increment_block)?;

        context.set_basic_block(increment_block);
        term.finalizer.wrap().into_llvm(context)?;
        context.build_unconditional_branch(condition_block)?;

        context.pop_loop();
        context.set_basic_block(join_block);

        Ok(())
    }
}

impl<D> era_compiler_llvm_context::EVMWriteLLVM<D> for WrappedForLoop
where
    D: era_compiler_llvm_context::Dependency,
{
    fn into_llvm(
        self,
        context: &mut era_compiler_llvm_context::EVMContext<D>,
    ) -> anyhow::Result<()> {
        self.0.initializer.wrap().into_llvm(context)?;

        let condition_block = context.append_basic_block("for_condition");
        let body_block = context.append_basic_block("for_body");
        let increment_block = context.append_basic_block("for_increment");
        let join_block = context.append_basic_block("for_join");

        context.build_unconditional_branch(condition_block)?;
        context.set_basic_block(condition_block);
        let condition = WrappedExpression(self.0.condition)
            .into_llvm_evm(context)?
            .expect("Always exists")
            .to_llvm()
            .into_int_value();
        let condition = context.builder().build_int_z_extend_or_bit_cast(
            condition,
            context.field_type(),
            "for_condition_extended",
        )?;
        let condition = context.builder().build_int_compare(
            inkwell::IntPredicate::NE,
            condition,
            context.field_const(0),
            "for_condition_compared",
        )?;
        context.build_conditional_branch(condition, body_block, join_block)?;

        context.push_loop(body_block, increment_block, join_block);

        context.set_basic_block(body_block);
        self.0.body.wrap().into_llvm(context)?;
        context.build_unconditional_branch(increment_block)?;

        context.set_basic_block(increment_block);
        self.0.finalizer.wrap().into_llvm(context)?;
        context.build_unconditional_branch(condition_block)?;

        context.pop_loop();
        context.set_basic_block(join_block);

        Ok(())
    }
}
