//!
//! The `solc --standard-json` input.
//!

pub mod language;
pub mod settings;
pub mod source;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;

use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;

use crate::standard_json::input::settings::codegen::Codegen as StandardJsonInputSettingsCodegen;
use crate::standard_json::input::settings::error_type::ErrorType as StandardJsonInputSettingsErrorType;
use crate::standard_json::input::settings::libraries::Libraries as StandardJsonInputSettingsLibraries;
use crate::standard_json::input::settings::metadata::Metadata as StandardJsonInputSettingsMetadata;
use crate::standard_json::input::settings::optimizer::Optimizer as StandardJsonInputSettingsOptimizer;
use crate::standard_json::input::settings::selection::Selection as StandardJsonInputSettingsSelection;
use crate::standard_json::input::settings::warning_type::WarningType as StandardJsonInputSettingsWarningType;

use self::language::Language;
use self::settings::Settings;
use self::source::Source;

///
/// The `solc --standard-json` input.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    /// The input language.
    pub language: Language,
    /// The input source code files hashmap.
    pub sources: BTreeMap<String, Source>,
    /// The compiler settings.
    pub settings: Settings,

    /// The suppressed errors.
    #[serde(default, skip_serializing)]
    pub suppressed_errors: Vec<StandardJsonInputSettingsErrorType>,
    /// The suppressed warnings.
    #[serde(default, skip_serializing)]
    pub suppressed_warnings: Vec<StandardJsonInputSettingsWarningType>,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    /// If the `path` is `None`, the input is read from the stdin.
    ///
    pub fn try_from(path: Option<&Path>) -> anyhow::Result<Self> {
        let input_json = match path {
            Some(path) => std::fs::read_to_string(path)
                .map_err(|error| anyhow::anyhow!("Standard JSON file {path:?} reading: {error}")),
            None => std::io::read_to_string(std::io::stdin())
                .map_err(|error| anyhow::anyhow!("Standard JSON reading from stdin: {error}")),
        }?;
        era_compiler_common::deserialize_from_str::<Self>(input_json.as_str())
            .map_err(|error| anyhow::anyhow!("Standard JSON parsing: {error}"))
    }

    ///
    /// A shortcut constructor from Solidity source paths.
    ///
    pub fn try_from_solidity_paths(
        paths: &[PathBuf],
        libraries: &[String],
        remappings: BTreeSet<String>,
        optimizer: StandardJsonInputSettingsOptimizer,
        codegen: Option<StandardJsonInputSettingsCodegen>,
        evm_version: Option<era_compiler_common::EVMVersion>,
        enable_eravm_extensions: bool,
        output_selection: StandardJsonInputSettingsSelection,
        metadata: StandardJsonInputSettingsMetadata,
        llvm_options: Vec<String>,
        suppressed_errors: Vec<StandardJsonInputSettingsErrorType>,
        suppressed_warnings: Vec<StandardJsonInputSettingsWarningType>,
        detect_missing_libraries: bool,
        via_ir: bool,
    ) -> anyhow::Result<Self> {
        let mut paths: BTreeSet<PathBuf> = paths.iter().cloned().collect();
        let libraries = StandardJsonInputSettingsLibraries::try_from(libraries)?;
        for library_file in libraries.as_inner().keys() {
            paths.insert(PathBuf::from(library_file));
        }

        let sources = paths
            .into_par_iter()
            .map(|path| {
                let source = Source::try_read(path.as_path())?;
                Ok((path.to_string_lossy().to_string(), source))
            })
            .collect::<anyhow::Result<BTreeMap<String, Source>>>()?;

        Self::try_from_solidity_sources(
            sources,
            libraries,
            remappings,
            optimizer,
            codegen,
            evm_version,
            enable_eravm_extensions,
            output_selection,
            metadata,
            llvm_options,
            suppressed_errors,
            suppressed_warnings,
            detect_missing_libraries,
            via_ir,
        )
    }

    ///
    /// A shortcut constructor from Solidity source code.
    ///
    pub fn try_from_solidity_sources(
        sources: BTreeMap<String, Source>,
        libraries: StandardJsonInputSettingsLibraries,
        remappings: BTreeSet<String>,
        optimizer: StandardJsonInputSettingsOptimizer,
        codegen: Option<StandardJsonInputSettingsCodegen>,
        evm_version: Option<era_compiler_common::EVMVersion>,
        enable_eravm_extensions: bool,
        output_selection: StandardJsonInputSettingsSelection,
        metadata: StandardJsonInputSettingsMetadata,
        llvm_options: Vec<String>,
        suppressed_errors: Vec<StandardJsonInputSettingsErrorType>,
        suppressed_warnings: Vec<StandardJsonInputSettingsWarningType>,
        detect_missing_libraries: bool,
        via_ir: bool,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            language: Language::Solidity,
            sources,
            settings: Settings::new(
                optimizer,
                libraries,
                remappings,
                codegen,
                evm_version,
                enable_eravm_extensions,
                output_selection,
                metadata,
                llvm_options,
                suppressed_errors.clone(),
                suppressed_warnings.clone(),
                detect_missing_libraries,
                via_ir,
            ),
            suppressed_errors,
            suppressed_warnings,
        })
    }

    ///
    /// A shortcut constructor from source code.
    ///
    pub fn from_yul_sources(
        sources: BTreeMap<String, Source>,
        libraries: StandardJsonInputSettingsLibraries,
        optimizer: StandardJsonInputSettingsOptimizer,
        llvm_options: Vec<String>,
    ) -> Self {
        let output_selection = StandardJsonInputSettingsSelection::new_yul_validation();

        Self {
            language: Language::Yul,
            sources,
            settings: Settings::new(
                optimizer,
                libraries,
                BTreeSet::new(),
                None,
                None,
                false,
                output_selection,
                StandardJsonInputSettingsMetadata::default(),
                llvm_options,
                vec![],
                vec![],
                false,
                false,
            ),
            suppressed_errors: vec![],
            suppressed_warnings: vec![],
        }
    }

    ///
    /// A shortcut constructor from source code.
    ///
    pub fn from_yul_paths(
        paths: &[PathBuf],
        libraries: StandardJsonInputSettingsLibraries,
        optimizer: StandardJsonInputSettingsOptimizer,
        llvm_options: Vec<String>,
    ) -> Self {
        let sources = paths
            .iter()
            .map(|path| {
                (
                    path.to_string_lossy().to_string(),
                    Source::from(path.as_path()),
                )
            })
            .collect();
        Self::from_yul_sources(sources, libraries, optimizer, llvm_options)
    }

    ///
    /// Extends the output selection with another one.
    ///
    pub fn extend_selection(&mut self, selection: StandardJsonInputSettingsSelection) {
        self.settings.extend_selection(selection);
    }

    ///
    /// Tries to resolve all sources.
    ///
    pub fn resolve_sources(&mut self) {
        self.sources
            .par_iter_mut()
            .map(|(_path, source)| {
                let _ = source.try_resolve();
            })
            .collect::<Vec<()>>();
    }
}
