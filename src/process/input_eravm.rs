//!
//! Process for compiling a single compilation unit.
//!
//! The EraVM input data.
//!

use crate::project::contract::Contract;
use crate::project::dependency_data::DependencyData;

///
/// The EraVM input data.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Input {
    /// The input contract.
    pub contract: Option<Contract>,
    /// The dependency data.
    pub dependency_data: DependencyData,
    /// Whether to enable EraVM extensions.
    pub enable_eravm_extensions: bool,
    /// Whether to append the metadata hash.
    pub include_metadata_hash: bool,
    /// Enables the test bytecode encoding.
    pub enable_test_encoding: bool,
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
        contract: Option<Contract>,
        dependency_data: DependencyData,
        enable_eravm_extensions: bool,
        include_metadata_hash: bool,
        enable_test_encoding: bool,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> Self {
        Self {
            contract,
            dependency_data,
            enable_eravm_extensions,
            include_metadata_hash,
            enable_test_encoding,
            optimizer_settings,
            llvm_options,
            debug_config,
        }
    }
}
