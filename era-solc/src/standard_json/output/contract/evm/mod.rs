//!
//! The `solc --standard-json` output contract EVM data.
//!

pub mod bytecode;
pub mod extra_metadata;

use std::collections::BTreeMap;

use self::bytecode::Bytecode;
use self::extra_metadata::ExtraMetadata;

///
/// The `solc --standard-json` output contract EVM data.
///
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EVM {
    /// The contract bytecode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytecode: Option<Bytecode>,
    /// The contract EVM legacy assembly code.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub legacy_assembly: serde_json::Value,
    /// The contract function signatures.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub method_identifiers: BTreeMap<String, String>,

    /// The contract EraVM assembly code.
    #[serde(default, skip_serializing_if = "Option::is_none", skip_deserializing)]
    pub assembly: Option<String>,
    /// The extra EVMLA metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_metadata: Option<ExtraMetadata>,
}

impl EVM {
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
            && self.legacy_assembly.is_null()
            && self.method_identifiers.is_empty()
            && self.assembly.is_none()
            && self.extra_metadata.is_none()
    }
}
