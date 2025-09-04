//!
//! The for-loop statement.
//!

use era_compiler_llvm_context::IContext;

use crate::declare_wrapper;
use crate::yul::parser::dialect::era::EraDialect;
use crate::yul::parser::wrapper::Wrap;

declare_wrapper!(
    era_yul::yul::parser::statement::for_loop::ForLoop<EraDialect>,
    ForLoop
);

impl era_compiler_llvm_context::EraVMWriteLLVM for ForLoop {
    fn into_llvm(
        self,
        context: &mut era_compiler_llvm_context::EraVMContext,
    ) -> anyhow::Result<()> {
        let term = self.0;
        term.initializer.wrap().into_llvm(context)?;

        let condition_block = context.append_basic_block("for_condition");
        let body_block = context.append_basic_block("for_body");
        let increment_block = context.append_basic_block("for_increment");
        let join_block = context.append_basic_block("for_join");

        context.build_unconditional_branch(condition_block)?;
        context.set_basic_block(condition_block);
        let condition = term
            .condition
            .wrap()
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
