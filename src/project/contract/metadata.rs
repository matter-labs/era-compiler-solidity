//!
//! The Solidity contract metadata.
//!

use serde::Serialize;

///
/// The Solidity contract metadata.
///
/// Is used to append the metadata hash to the contract bytecode.
///
#[derive(Debug, Serialize)]
pub struct Metadata {
    /// The `solc` metadata.
    pub solc_metadata: serde_json::Value,
    /// The zkEVM compiler version.
    pub zk_version: semver::Version,
    /// The zkEVM compiler stringified optimizer settings.
    pub optimizer_settings: String,
}

impl Metadata {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        solc_metadata: serde_json::Value,
        zk_version: semver::Version,
        optimizer_settings: compiler_llvm_context::OptimizerSettings,
    ) -> Self {
        Self {
            solc_metadata,
            zk_version,
            optimizer_settings: optimizer_settings.to_string(),
        }
    }
}
