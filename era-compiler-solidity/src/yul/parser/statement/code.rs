//!
//! The YUL code.
//!

use crate::create_wrapper;
use crate::yul::parser::dialect::era::EraDialect;
use crate::yul::parser::wrapper::Wrap as _;

create_wrapper!(
    era_yul::yul::parser::statement::code::Code<EraDialect>,
    WrappedCode
);

impl<D> era_compiler_llvm_context::EraVMWriteLLVM<D> for WrappedCode
where
    D: era_compiler_llvm_context::Dependency,
{
    fn into_llvm(
        self,
        context: &mut era_compiler_llvm_context::EraVMContext<D>,
    ) -> anyhow::Result<()> {
        self.0.block.wrap().into_llvm(context)?;

        Ok(())
    }
}

impl<D> era_compiler_llvm_context::EVMWriteLLVM<D> for WrappedCode
where
    D: era_compiler_llvm_context::Dependency,
{
    fn into_llvm(
        self,
        context: &mut era_compiler_llvm_context::EVMContext<D>,
    ) -> anyhow::Result<()> {
        self.0.block.wrap().into_llvm(context)?;

        Ok(())
    }
}
