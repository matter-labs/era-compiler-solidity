//!
//! The `solc --standard-json` output contract EVM data.
//!

pub mod bytecode;
pub mod extra_metadata;

use std::collections::BTreeMap;

use crate::evmla::assembly::Assembly;

use self::bytecode::Bytecode;
use self::extra_metadata::ExtraMetadata;

///
/// The `solc --standard-json` output contract EVM data.
///
/// It is replaced by EraVM data after compiling.
///
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EVM {
    /// The contract bytecode.
    /// Is reset by that of EraVM before yielding the compiled project artifacts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytecode: Option<Bytecode>,
    /// The contract EVM legacy assembly code.
    #[serde(rename = "legacyAssembly", skip_serializing_if = "Option::is_none")]
    pub legacy_assembly: Option<Assembly>,
    /// The contract function signatures.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method_identifiers: Option<BTreeMap<String, String>>,

    /// The contract EraVM assembly code.
    #[serde(rename = "assembly", skip_serializing_if = "Option::is_none")]
    pub assembly: Option<String>,
    /// The extra EVMLA metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_metadata: Option<ExtraMetadata>,
}

impl EVM {
    ///
    /// A shortcut constructor for EraVM.
    ///
    pub fn new_eravm(bytecode: String, assembly: Option<String>) -> Self {
        Self {
            bytecode: Some(Bytecode::new(bytecode)),
            legacy_assembly: None,
            method_identifiers: None,

            assembly,
            extra_metadata: None,
        }
    }

    ///
    /// Sets the EraVM assembly and bytecode.
    ///
    pub fn modify_eravm(&mut self, bytecode: String, assembly: Option<String>) {
        self.bytecode = Some(Bytecode::new(bytecode));
        self.assembly = assembly;
    }

    ///
    /// Sets the EVM and deploy and runtime bytecode.
    ///
    pub fn modify_evm(&mut self, deploy_bytecode: String, runtime_bytecode: String) {
        let mut bytecode = deploy_bytecode;
        bytecode.push_str(runtime_bytecode.as_str());
        self.bytecode = Some(Bytecode::new(bytecode));
    }

    ///
    /// Checks if all fields are `None`.
    ///
    pub fn is_empty(&self) -> bool {
        self.bytecode.is_none()
            && self.legacy_assembly.is_none()
            && self.method_identifiers.is_none()
            && self.assembly.is_none()
            && self.extra_metadata.is_none()
    }
}
