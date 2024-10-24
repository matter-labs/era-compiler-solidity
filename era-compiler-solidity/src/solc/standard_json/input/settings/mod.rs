//!
//! The `solc --standard-json` input settings.
//!

pub mod metadata;
pub mod optimizer;
pub mod selection;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashSet;

use crate::error_type::ErrorType;
use crate::solc::codegen::Codegen as SolcCodegen;
use crate::warning_type::WarningType;

use self::metadata::Metadata;
use self::optimizer::Optimizer;
use self::selection::file::flag::Flag as SelectionFlag;
use self::selection::Selection;

///
/// The `solc --standard-json` input settings.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// The optimizer settings.
    #[serde(default)]
    pub optimizer: Optimizer,

    /// The linker library addresses.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub libraries: BTreeMap<String, BTreeMap<String, String>>,
    /// The sorted list of remappings.
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub remappings: BTreeSet<String>,

    /// The target EVM version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evm_version: Option<era_compiler_common::EVMVersion>,
    /// The output selection filters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_selection: Option<Selection>,
    /// The metadata settings.
    #[serde(default)]
    pub metadata: Metadata,

    /// The Solidity codegen.
    #[serde(skip_serializing)]
    pub codegen: Option<SolcCodegen>,
    /// Whether to compile via EVM assembly.
    #[serde(default, rename = "forceEVMLA", skip_serializing)]
    pub force_evmla: bool,
    /// Whether to enable EraVM extensions.
    #[serde(default, rename = "enableEraVMExtensions", skip_serializing)]
    pub enable_eravm_extensions: bool,

    /// The extra LLVM options.
    #[serde(default, rename = "LLVMOptions", skip_serializing)]
    pub llvm_options: Vec<String>,
    /// The suppressed errors.
    #[serde(default, skip_serializing)]
    pub suppressed_errors: Vec<ErrorType>,
    /// The suppressed warnings.
    #[serde(default, skip_serializing)]
    pub suppressed_warnings: Vec<WarningType>,

    /// Whether to enable the missing libraries detection mode.
    /// Deprecated in favor of post-compile-time linking.
    #[serde(default, rename = "detectMissingLibraries", skip_serializing)]
    pub detect_missing_libraries: bool,
    /// Whether to add the Yul step to compilation via EVM assembly.
    /// Only used from era-compiler-tester to allow running additional tests.
    #[serde(
        rename = "viaIR",
        skip_deserializing,
        skip_serializing_if = "Option::is_none"
    )]
    pub via_ir: Option<bool>,
}

impl Settings {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        optimizer: Optimizer,

        libraries: BTreeMap<String, BTreeMap<String, String>>,
        remappings: BTreeSet<String>,

        codegen: Option<SolcCodegen>,
        evm_version: Option<era_compiler_common::EVMVersion>,
        enable_eravm_extensions: bool,

        output_selection: Selection,
        metadata: Metadata,
        llvm_options: Vec<String>,
        suppressed_errors: Vec<ErrorType>,
        suppressed_warnings: Vec<WarningType>,

        detect_missing_libraries: bool,
        via_ir: bool,
    ) -> Self {
        Self {
            optimizer,

            libraries,
            remappings,

            codegen,
            evm_version,
            force_evmla: codegen == Some(SolcCodegen::EVMLA),
            enable_eravm_extensions,

            output_selection: Some(output_selection),
            metadata,
            llvm_options,
            suppressed_errors,
            suppressed_warnings,

            detect_missing_libraries,
            via_ir: if via_ir { Some(true) } else { None },
        }
    }

    ///
    /// Sets the necessary defaults for EraVM compilation.
    ///
    pub fn normalize(&mut self, pipeline: Option<SolcCodegen>) {
        self.output_selection
            .get_or_insert_with(Selection::default)
            .extend_with_required(pipeline);
    }

    ///
    /// Sets the necessary defaults for Yul validation.
    ///
    pub fn normalize_yul_validation(&mut self) {
        self.output_selection
            .get_or_insert_with(Selection::new_yul_validation)
            .extend_with_yul_validation();
    }

    ///
    /// Returns flags that are going to be automatically added by the compiler,
    /// but were not explicitly requested by the user.
    ///
    /// Afterwards, the flags are used to prune JSON output before returning it.
    ///
    pub fn get_unset_required(&self) -> HashSet<SelectionFlag> {
        self.output_selection
            .as_ref()
            .map(|selection| selection.get_unset_required())
            .unwrap_or_else(|| Selection::default().get_unset_required())
    }
}
