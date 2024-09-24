//!
//! Process for compiling a single compilation unit.
//!
//! The EraVM input data.
//!

pub mod dependency_data;

use crate::project::contract::Contract;

use self::dependency_data::DependencyData;

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
        contract: Option<Contract>,
        dependency_data: DependencyData,
        enable_eravm_extensions: bool,
        metadata_hash_type: era_compiler_common::HashType,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        output_assembly: bool,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> Self {
        Self {
            contract,
            dependency_data,
            enable_eravm_extensions,
            metadata_hash_type,
            optimizer_settings,
            llvm_options,
            output_assembly,
            debug_config,
        }
    }
}
