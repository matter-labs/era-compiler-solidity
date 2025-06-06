//!
//! Process for compiling a single compilation unit.
//!
//! The EVM input data.
//!

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::project::contract::Contract;

///
/// The EVM input data.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Input {
    /// The input contract.
    pub contract: Contract,
    /// The `solc` compiler version.
    pub solc_version: Option<era_solc::Version>,
    /// The mapping of auxiliary identifiers, e.g. Yul object names, to full contract paths.
    pub identifier_paths: BTreeMap<String, String>,
    /// Missing unlinked libraries.
    pub missing_libraries: BTreeSet<String>,
    /// The metadata hash type.
    pub metadata_hash_type: era_compiler_common::EVMMetadataHashType,
    /// Append the CBOR metadata at the end of bytecode.
    pub append_cbor: bool,
    /// The optimizer settings.
    pub optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    /// The extra LLVM arguments.
    pub llvm_options: Vec<String>,
    /// The debug output config.
    pub debug_config: Option<era_compiler_llvm_context::DebugConfig>,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        contract: Contract,
        solc_version: Option<era_solc::Version>,
        identifier_paths: BTreeMap<String, String>,
        missing_libraries: BTreeSet<String>,
        metadata_hash_type: era_compiler_common::EVMMetadataHashType,
        append_cbor: bool,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> Self {
        Self {
            contract,
            solc_version,
            identifier_paths,
            missing_libraries,
            metadata_hash_type,
            append_cbor,
            optimizer_settings,
            llvm_options,
            debug_config,
        }
    }
}
