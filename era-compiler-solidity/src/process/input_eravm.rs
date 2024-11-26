//!
//! Process for compiling a single compilation unit.
//!
//! The EraVM input data.
//!

use std::collections::BTreeMap;

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
    /// The linker symbols.
    pub linker_symbols: BTreeMap<String, [u8; era_compiler_common::BYTE_LENGTH_ETH_ADDRESS]>,
    /// The metadata hash type.
    pub metadata_hash_type: era_compiler_common::HashType,
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
        enable_eravm_extensions: bool,
        linker_symbols: BTreeMap<String, [u8; era_compiler_common::BYTE_LENGTH_ETH_ADDRESS]>,
        metadata_hash_type: era_compiler_common::HashType,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        output_assembly: bool,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> Self {
        Self {
            contract,
            solc_version,
            identifier_paths,
            enable_eravm_extensions,
            linker_symbols,
            metadata_hash_type,
            optimizer_settings,
            llvm_options,
            output_assembly,
            debug_config,
        }
    }
}
