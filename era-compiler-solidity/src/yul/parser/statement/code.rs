//!
//! The Yul code.
//!

use crate::declare_wrapper;
use crate::yul::parser::dialect::era::EraDialect;
use crate::yul::parser::wrapper::Wrap;

declare_wrapper!(
    era_yul::yul::parser::statement::code::Code<EraDialect>,
    Code
);

impl era_compiler_llvm_context::EraVMWriteLLVM for Code {
    fn into_llvm(
        self,
        context: &mut era_compiler_llvm_context::EraVMContext,
    ) -> anyhow::Result<()> {
        self.0.block.wrap().into_llvm(context)?;

        Ok(())
    }
}
