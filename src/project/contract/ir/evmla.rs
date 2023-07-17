//!
//! The contract EVM legacy assembly source code.
//!

use serde::Deserialize;
use serde::Serialize;

use crate::evmla::assembly::Assembly;
use crate::solc::standard_json::output::contract::evm::extra_metadata::ExtraMetadata;

///
/// The contract EVM legacy assembly source code.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub fn new(mut assembly: Assembly, extra_metadata: ExtraMetadata) -> Self {
        assembly.extra_metadata = Some(extra_metadata);
        Self { assembly }
    }
}

impl<D> compiler_llvm_context::WriteLLVM<D> for EVMLA
where
    D: compiler_llvm_context::Dependency + Clone,
{
    fn declare(&mut self, context: &mut compiler_llvm_context::Context<D>) -> anyhow::Result<()> {
        self.assembly.declare(context)
    }

    fn into_llvm(self, context: &mut compiler_llvm_context::Context<D>) -> anyhow::Result<()> {
        self.assembly.into_llvm(context)
    }
}
