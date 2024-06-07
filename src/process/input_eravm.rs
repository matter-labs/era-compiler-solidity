//!
//! Process for compiling a single compilation unit.
//!
//! The EraVM input data.
//!

use std::borrow::Cow;

use serde::Deserialize;
use serde::Serialize;

use crate::project::contract::Contract;
use crate::project::Project;

///
/// The EraVM input data.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Input<'a> {
    /// The contract representation.
    pub contract: Cow<'a, Contract>,
    /// The project representation.
    pub project: Cow<'a, Project>,
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

impl<'a> Input<'a> {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        contract: Cow<'a, Contract>,
        project: Cow<'a, Project>,
        enable_eravm_extensions: bool,
        include_metadata_hash: bool,
        enable_test_encoding: bool,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> Self {
        Self {
            contract,
            project,
            enable_eravm_extensions,
            include_metadata_hash,
            enable_test_encoding,
            optimizer_settings,
            llvm_options,
            debug_config,
        }
    }
}
