//!
//! The `solc --combined-json` contract.
//!

use std::collections::BTreeMap;
use std::collections::HashSet;

///
/// The contract.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Contract {
    /// The `solc` hashes output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashes: Option<BTreeMap<String, String>>,
    /// The `solc` ABI output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi: Option<serde_json::Value>,
    /// The `solc` metadata output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    /// The `solc` developer documentation output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devdoc: Option<serde_json::Value>,
    /// The `solc` user documentation output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userdoc: Option<serde_json::Value>,
    /// The `solc` storage layout output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_layout: Option<serde_json::Value>,
    /// The `solc` AST output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ast: Option<serde_json::Value>,
    /// The `solc` assembly output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asm: Option<serde_json::Value>,
    /// The `solc` hexadecimal binary output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<String>,
    /// The `solc` hexadecimal binary runtime part output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_runtime: Option<String>,

    /// The EraVM assembly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assembly: Option<serde_json::Value>,
    /// The factory dependencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factory_deps: Option<BTreeMap<String, String>>,
    /// The missing libraries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_libraries: Option<HashSet<String>>,
}
