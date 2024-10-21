//!
//! The contract EVM legacy assembly source code.
//!

use std::collections::HashSet;

use crate::evmla::assembly::Assembly;
use crate::solc::standard_json::output::contract::Contract as SolcStandardJsonOutputContract;

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
    pub fn try_from_contract(contract: &SolcStandardJsonOutputContract) -> Option<Self> {
        let evm = contract.evm.as_ref()?;
        let extra_metadata = evm.extra_metadata.clone().unwrap_or_default();

        let mut assembly = evm.legacy_assembly.as_ref()?.to_owned();
        assembly.extra_metadata = Some(extra_metadata);

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
