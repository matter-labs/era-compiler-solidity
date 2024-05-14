//!
//! The Solidity compiler pipeline type.
//!

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

impl TryFrom<(Option<bool>, Option<bool>)> for Pipeline {
    type Error = anyhow::Error;

    fn try_from((via_evm_assembly, via_ir): (Option<bool>, Option<bool>)) -> anyhow::Result<Self> {
        match (via_evm_assembly, via_ir) {
            (Some(true), Some(true)) => anyhow::bail!("Conflicting codegen settings: consider removing the `viaEVMAssembly` or `viaIR` field from the JSON input."),
            (Some(true), _) => Ok(Self::EVMLA),
            (_, Some(true)) => Ok(Self::Yul),
            (_, _) => anyhow::bail!("Missing codegen settings: consider adding the `viaEVMAssembly` or `viaIR` field to the JSON input."),
        }
    }
}
