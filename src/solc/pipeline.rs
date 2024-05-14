//!
//! The Solidity compiler pipeline type.
//!

use crate::solc::version::Version as SolcVersion;
use crate::solc::Compiler as SolcCompiler;

///
/// The Solidity compiler pipeline type.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pipeline {
    /// The Yul IR.
    Yul,
    /// The EVM legacy assembly IR.
    EVMLA,
}

impl Pipeline {
    ///
    /// A shortcut constructor with validation.
    ///
    pub fn new(
        via_evm_assembly: Option<bool>,
        via_yul: Option<bool>,
        solc_version: &SolcVersion,
    ) -> anyhow::Result<Self> {
        match (via_evm_assembly, via_yul) {
            (Some(true), Some(true)) => anyhow::bail!("Conflicting codegen settings: consider removing the `viaEVMAssembly` or `viaIR` field from the JSON input."),
            (Some(true), _) if solc_version.l2_revision.is_none() => anyhow::bail!("EVM assembly codegen is only supported with the ZKsync edition of solc."),
            (Some(true), _) => Ok(Self::EVMLA),
            (_, Some(true)) if solc_version.default < SolcCompiler::FIRST_YUL_VERSION => anyhow::bail!("Yul codegen is only supported for solc >= `{}`.", SolcCompiler::FIRST_YUL_VERSION),
            (_, Some(true)) => Ok(Self::Yul),
            (_, _) => anyhow::bail!("Missing codegen settings: consider adding the `viaEVMAssembly` or `viaIR` field to the JSON input."),
        }
    }
}
