//!
//! The contract metadata.
//!

///
/// The contract metadata.
///
/// Is used to append the metadata hash to the contract bytecode.
///
#[derive(Debug, serde::Serialize)]
pub struct Metadata<'a> {
    /// The source code metadata.
    /// If `solc` is used in the pipeline, its metadata is used here.
    pub source_metadata: serde_json::Value,
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
        source_metadata: serde_json::Value,
        solc_version: Option<semver::Version>,
        solc_zkvm_edition: Option<semver::Version>,
        zk_version: semver::Version,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: &'a [String],
    ) -> Self {
        Self {
            source_metadata,
            solc_version,
            solc_zkvm_edition,
            zk_version,
            optimizer_settings,
            llvm_options,
        }
    }
}
