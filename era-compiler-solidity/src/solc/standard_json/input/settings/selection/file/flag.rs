//!
//! The `solc --standard-json` expected output selection flag.
//!

use crate::solc::codegen::Codegen as SolcCodegen;

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

impl From<SolcCodegen> for Flag {
    fn from(codegen: SolcCodegen) -> Self {
        match codegen {
            SolcCodegen::Yul => Self::Yul,
            SolcCodegen::EVMLA => Self::EVMLA,
        }
    }
}
