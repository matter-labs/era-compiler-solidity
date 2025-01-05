//!
//! The `solc --standard-json` expected output selector.
//!

use crate::standard_json::input::settings::codegen::Codegen as StandardJsonInputSettingsCodegen;

///
/// The `solc --standard-json` expected output selector.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Selector {
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
    #[serde(rename = "eravm.assembly", skip_serializing)]
    EraVMAssembly,
}

impl From<StandardJsonInputSettingsCodegen> for Selector {
    fn from(codegen: StandardJsonInputSettingsCodegen) -> Self {
        match codegen {
            StandardJsonInputSettingsCodegen::Yul => Self::Yul,
            StandardJsonInputSettingsCodegen::EVMLA => Self::EVMLA,
        }
    }
}
