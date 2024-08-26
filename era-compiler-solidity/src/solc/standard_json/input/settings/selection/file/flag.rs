//!
//! The `solc --standard-json` expected output selection flag.
//!

use crate::solc::pipeline::Pipeline as SolcPipeline;

///
/// The `solc --standard-json` expected output selection flag.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Flag {
    /// The ABI JSON.
    #[serde(rename = "abi")]
    ABI,
    /// The metadata.
    #[serde(rename = "metadata")]
    Metadata,
    /// The developer documentation.
    #[serde(rename = "devdoc")]
    Devdoc,
    /// The user documentation.
    #[serde(rename = "userdoc")]
    Userdoc,
    /// The storage layout.
    #[serde(rename = "storageLayout")]
    StorageLayout,
    /// The AST JSON.
    #[serde(rename = "ast")]
    AST,
    /// The Yul IR.
    #[serde(rename = "irOptimized")]
    Yul,
    /// The EVM bytecode.
    #[serde(rename = "evm")]
    EVM,
    /// The EVM legacy assembly JSON.
    #[serde(rename = "evm.legacyAssembly")]
    EVMLA,
    /// The function signature hashes JSON.
    #[serde(rename = "evm.methodIdentifiers")]
    MethodIdentifiers,
    /// The EraVM assembly.
    #[serde(rename = "eravm.assembly")]
    EraVMAssembly,
}

impl From<SolcPipeline> for Flag {
    fn from(pipeline: SolcPipeline) -> Self {
        match pipeline {
            SolcPipeline::Yul => Self::Yul,
            SolcPipeline::EVMLA => Self::EVMLA,
        }
    }
}

impl std::fmt::Display for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ABI => write!(f, "abi"),
            Self::Metadata => write!(f, "metadata"),
            Self::Devdoc => write!(f, "devdoc"),
            Self::Userdoc => write!(f, "userdoc"),
            Self::StorageLayout => write!(f, "storageLayout"),
            Self::AST => write!(f, "ast"),
            Self::Yul => write!(f, "irOptimized"),
            Self::EVM => write!(f, "evm"),
            Self::EVMLA => write!(f, "evm.legacyAssembly"),
            Self::MethodIdentifiers => write!(f, "evm.methodIdentifiers"),
            Self::EraVMAssembly => write!(f, "eravm.assembly"),
        }
    }
}
