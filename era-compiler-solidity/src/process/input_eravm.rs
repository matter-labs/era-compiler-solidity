//!
//! Process for compiling a single compilation unit.
//!
//! The EraVM input data.
//!

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::project::contract::Contract;

///
/// The EraVM input data.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Input {
    /// The input contract.
    pub contract: Contract,
    /// The `solc` compiler version.
    pub solc_version: Option<era_solc::Version>,
    /// The mapping of auxiliary identifiers, e.g. Yul object names, to full contract paths.
    pub identifier_paths: BTreeMap<String, String>,
    /// Whether to enable EraVM extensions.
    pub enable_eravm_extensions: bool,
    /// Missing unlinked libraries.
    pub missing_libraries: BTreeSet<String>,
    /// Factory dependencies.
    pub factory_dependencies: BTreeSet<String>,
    /// The metadata hash type.
    pub metadata_hash_type: era_compiler_common::EraVMMetadataHashType,
    /// Append the CBOR metadata at the end of bytecode.
    pub append_cbor: bool,
    /// The optimizer settings.
    pub optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
    /// The extra LLVM arguments.
    pub llvm_options: Vec<String>,
    /// Whether to output EraVM assembly.
    pub output_assembly: bool,
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
        factory_dependencies: BTreeSet<String>,
        enable_eravm_extensions: bool,
        metadata_hash_type: era_compiler_common::EraVMMetadataHashType,
        append_cbor: bool,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        output_assembly: bool,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> Self {
        Self {
            contract,
            solc_version,
            identifier_paths,
            missing_libraries,
            factory_dependencies,
            enable_eravm_extensions,
            metadata_hash_type,
            append_cbor,
            optimizer_settings,
            llvm_options,
            output_assembly,
            debug_config,
        }
    }
}
