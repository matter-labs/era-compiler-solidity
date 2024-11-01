//!
//! The contract EVM legacy assembly source code.
//!

use std::collections::HashSet;

use crate::evmla::assembly::Assembly;

///
/// The contract EVM legacy assembly source code.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EVMLA {
    /// The EVM legacy assembly source code.
    pub assembly: Assembly,
}

impl EVMLA {
    ///
    /// Transforms the `solc` standard JSON output contract into an EVM legacy assembly object.
    ///
    pub fn try_from_contract(contract: &era_solc::StandardJsonOutputContract) -> Option<Self> {
        let evm = contract.evm.as_ref()?;

        let mut assembly: Assembly = serde_json::from_value(evm.legacy_assembly.to_owned()).ok()?;
        assembly.extra_metadata = evm.extra_metadata.to_owned();

        Some(Self { assembly })
    }

    ///
    /// Get the list of missing deployable libraries.
    ///
    pub fn get_missing_libraries(&self) -> HashSet<String> {
        self.assembly.get_missing_libraries()
    }
}

impl<D> era_compiler_llvm_context::EraVMWriteLLVM<D> for EVMLA
where
    D: era_compiler_llvm_context::Dependency + Clone,
{
    fn declare(
        &mut self,
        context: &mut era_compiler_llvm_context::EraVMContext<D>,
    ) -> anyhow::Result<()> {
        self.assembly.declare(context)
    }

    fn into_llvm(
        self,
        context: &mut era_compiler_llvm_context::EraVMContext<D>,
    ) -> anyhow::Result<()> {
        self.assembly.into_llvm(context)
    }
}

impl<D> era_compiler_llvm_context::EVMWriteLLVM<D> for EVMLA
where
    D: era_compiler_llvm_context::Dependency + Clone,
{
    fn declare(
        &mut self,
        context: &mut era_compiler_llvm_context::EVMContext<D>,
    ) -> anyhow::Result<()> {
        self.assembly.declare(context)
    }

    fn into_llvm(
        self,
        context: &mut era_compiler_llvm_context::EVMContext<D>,
    ) -> anyhow::Result<()> {
        self.assembly.into_llvm(context)
    }
}
