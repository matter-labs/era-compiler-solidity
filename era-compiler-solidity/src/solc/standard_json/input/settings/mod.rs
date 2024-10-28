//!
//! The `solc --standard-json` input settings.
//!

pub mod codegen;
pub mod metadata;
pub mod optimizer;
pub mod selection;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::error_type::ErrorType;
use crate::warning_type::WarningType;

use self::codegen::Codegen;
use self::metadata::Metadata;
use self::optimizer::Optimizer;
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
    #[serde(default)]
    pub output_selection: Selection,
    /// The metadata settings.
    #[serde(default)]
    pub metadata: Metadata,

    /// The Solidity codegen.
    #[serde(skip_serializing)]
    pub codegen: Option<Codegen>,
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

        codegen: Option<Codegen>,
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
            force_evmla: codegen == Some(Codegen::EVMLA),
            enable_eravm_extensions,

            output_selection,
            metadata,
            llvm_options,
            suppressed_errors,
            suppressed_warnings,

            detect_missing_libraries,
            via_ir: if via_ir { Some(true) } else { None },
        }
    }

    ///
    /// Extends the output selection with another one.
    ///
    pub fn extend_selection(&mut self, selection: Selection) {
        self.output_selection.extend(selection);
    }

    ///
    /// Returns flags that are going to be automatically added by the compiler,
    /// but were not explicitly requested by the user.
    ///
    /// Afterwards, the flags are used to prune JSON output before returning it.
    ///
    pub fn selection_to_prune(&self) -> Selection {
        self.output_selection.selection_to_prune()
    }
}
