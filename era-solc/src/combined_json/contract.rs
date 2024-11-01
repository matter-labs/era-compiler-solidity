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
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub hashes: BTreeMap<String, String>,
    /// The `solc` ABI output.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub abi: serde_json::Value,
    /// The `solc` metadata output.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    /// The `solc` developer documentation output.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub devdoc: serde_json::Value,
    /// The `solc` user documentation output.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub userdoc: serde_json::Value,
    /// The `solc` storage layout output.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub storage_layout: serde_json::Value,
    /// The `solc` AST output.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub ast: serde_json::Value,
    /// The `solc` assembly output.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub asm: serde_json::Value,
    /// The `solc` hexadecimal binary output.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bin: Option<String>,
    /// The `solc` hexadecimal binary runtime part output.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bin_runtime: Option<String>,

    /// The EraVM assembly.
    #[serde(
        default,
        skip_serializing_if = "serde_json::Value::is_null",
        skip_deserializing
    )]
    pub assembly: serde_json::Value,
    /// The factory dependencies.
    #[serde(default, skip_deserializing)]
    pub factory_deps: BTreeMap<String, String>,
    /// The missing libraries.
    #[serde(default, skip_serializing_if = "HashSet::is_empty", skip_deserializing)]
    pub missing_libraries: HashSet<String>,
}
