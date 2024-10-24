//!
//! The `solc --standard-json` output contract.
//!

pub mod eravm;
pub mod evm;

use std::collections::BTreeMap;
use std::collections::HashSet;

use self::eravm::EraVM;
use self::evm::EVM;

///
/// The `solc --standard-json` output contract.
///
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    /// The contract ABI.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abi: Option<serde_json::Value>,
    /// The contract storage layout.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_layout: Option<serde_json::Value>,
    /// The contract storage layout.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transient_storage_layout: Option<serde_json::Value>,
    /// The contract metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// The contract developer documentation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devdoc: Option<serde_json::Value>,
    /// The contract user documentation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub userdoc: Option<serde_json::Value>,
    /// The contract optimized IR code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ir_optimized: Option<String>,
    /// The EraVM data of the contract.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eravm: Option<EraVM>,
    /// The EVM data of the contract.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evm: Option<EVM>,

    /// The contract EraVM bytecode hash.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    /// The contract factory dependencies.
    #[serde(default)]
    pub factory_dependencies: BTreeMap<String, String>,

    /// The contract missing libraries.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing_libraries: Option<HashSet<String>>,
}

impl Contract {
    ///
    /// Checks if all fields are unset or empty.
    ///
    pub fn is_empty(&self) -> bool {
        self.abi.is_none()
            && self.storage_layout.is_none()
            && self.transient_storage_layout.is_none()
            && self.metadata.is_none()
            && self.devdoc.is_none()
            && self.userdoc.is_none()
            && self.evm.is_none()
            && self.ir_optimized.is_none()
            && self.hash.is_none()
            && self.factory_dependencies.is_empty()
            && self.missing_libraries.is_none()
    }
}
