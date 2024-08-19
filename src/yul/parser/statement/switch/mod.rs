//!
//! The switch statement.
//!

use era_compiler_llvm_context::{EraVMWriteLLVM, IContext};

use crate::create_wrapper;
use crate::yul::parser::dialect::llvm::LLVMDialect;
use crate::yul::parser::wrapper::Wrap as _;

create_wrapper!(
    yul_syntax_tools::yul::parser::statement::switch::Switch<LLVMDialect>,
    WrappedSwitch
);

impl<D> EraVMWriteLLVM<D> for WrappedSwitch
where
    D: era_compiler_llvm_context::Dependency,
{
    fn into_llvm(
        self,
        context: &mut era_compiler_llvm_context::EraVMContext<D>,
    ) -> anyhow::Result<()> {
        let term = self.0;
        let scrutinee = term.expression.wrap().into_llvm(context)?;

        if term.cases.is_empty() {
            if let Some(block) = term.default {
                block.wrap().into_llvm(context)?;
            }
            return Ok(());
        }

        let current_block = context.basic_block();
        let join_block = context.append_basic_block("switch_join_block");

        let mut branches = Vec::with_capacity(term.cases.len());
        for (index, case) in term.cases.into_iter().enumerate() {
            let constant = case.literal.wrap().into_llvm(context)?.to_llvm();

            let expression_block = context
                .append_basic_block(format!("switch_case_branch_{}_block", index + 1).as_str());
            context.set_basic_block(expression_block);
            case.block.wrap().into_llvm(context)?;
            context.build_unconditional_branch(join_block)?;

            branches.push((constant.into_int_value(), expression_block));
        }

        let default_block = match term.default {
            Some(default) => {
                let default_block = context.append_basic_block("switch_default_block");
                context.set_basic_block(default_block);
                default.wrap().into_llvm(context)?;
                context.build_unconditional_branch(join_block)?;
                default_block
            }
            None => join_block,
        };

        context.set_basic_block(current_block);
        context.builder().build_switch(
            scrutinee.expect("Always exists").to_llvm().into_int_value(),
            default_block,
            branches.as_slice(),
        )?;

        context.set_basic_block(join_block);

        Ok(())
    }
}

impl<D> era_compiler_llvm_context::EVMWriteLLVM<D> for WrappedSwitch
where
    D: era_compiler_llvm_context::Dependency,
{
    fn into_llvm(
        self,
        context: &mut era_compiler_llvm_context::EVMContext<D>,
    ) -> anyhow::Result<()> {
        let scrutinee = self.0.expression.wrap().into_llvm_evm(context)?;

        if self.0.cases.is_empty() {
            if let Some(block) = self.0.default {
                era_compiler_llvm_context::EVMWriteLLVM::into_llvm(block.wrap(), context)?;
            }
            return Ok(());
        }

        let current_block = context.basic_block();
        let join_block = context.append_basic_block("switch_join_block");

        let mut branches = Vec::with_capacity(self.0.cases.len());
        for (index, case) in self.0.cases.into_iter().enumerate() {
            let constant = case.literal.wrap().into_llvm(context)?.to_llvm();

            let expression_block = context
                .append_basic_block(format!("switch_case_branch_{}_block", index + 1).as_str());
            context.set_basic_block(expression_block);
            era_compiler_llvm_context::EVMWriteLLVM::into_llvm(case.block.wrap(), context)?;
            context.build_unconditional_branch(join_block)?;

            branches.push((constant.into_int_value(), expression_block));
        }

        let default_block = match self.0.default {
            Some(default) => {
                let default_block = context.append_basic_block("switch_default_block");
                context.set_basic_block(default_block);
                era_compiler_llvm_context::EVMWriteLLVM::into_llvm(default.wrap(), context)?;
                context.build_unconditional_branch(join_block)?;
                default_block
            }
            None => join_block,
        };

        context.set_basic_block(current_block);
        context.builder().build_switch(
            scrutinee.expect("Always exists").to_llvm().into_int_value(),
            default_block,
            branches.as_slice(),
        )?;

        context.set_basic_block(join_block);

        Ok(())
    }
}
