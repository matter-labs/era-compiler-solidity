//!
//! The contract EVM legacy assembly source code.
//!

use crate::evmla::assembly::Assembly;

///
/// The contract EVM legacy assembly source code.
///
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
pub struct EVMLA {
    /// The EVM legacy assembly source code.
    pub assembly: Assembly,
}

impl EVMLA {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(assembly: Assembly) -> Self {
        Self { assembly }
    }
}

impl<D> compiler_llvm_context::WriteLLVM<D> for EVMLA
where
    D: compiler_llvm_context::Dependency,
{
    fn declare(&mut self, context: &mut compiler_llvm_context::Context<D>) -> anyhow::Result<()> {
        self.assembly.declare(context)
    }

    fn into_llvm(self, context: &mut compiler_llvm_context::Context<D>) -> anyhow::Result<()> {
        self.assembly.into_llvm(context)
    }
}
