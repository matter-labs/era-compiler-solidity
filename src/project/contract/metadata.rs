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
pub struct Metadata<'a> {
    /// The `solc` metadata.
    pub solc_metadata: serde_json::Value,
    /// The `solc` version if used.
    pub solc_version: Option<semver::Version>,
    /// The ZKsync `solc` edition.
    pub solc_zkvm_edition: Option<semver::Version>,
    /// The EraVM compiler version.
    pub zk_version: semver::Version,
    /// The EraVM compiler optimizer settings.
    pub optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    /// The LLVM extra arguments.
    pub llvm_options: &'a [String],
}

impl<'a> Metadata<'a> {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        solc_metadata: serde_json::Value,
        solc_version: Option<semver::Version>,
        solc_zkvm_edition: Option<semver::Version>,
        zk_version: semver::Version,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: &'a [String],
    ) -> Self {
        Self {
            solc_metadata,
            solc_version,
            solc_zkvm_edition,
            zk_version,
            optimizer_settings,
            llvm_options,
        }
    }
}
