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

use crate::message_type::MessageType;
use crate::solc::pipeline::Pipeline as SolcPipeline;
use crate::solc::standard_json::input::settings::metadata::Metadata as SolcStandardJsonInputSettingsMetadata;
use crate::solc::standard_json::input::settings::optimizer::Optimizer as SolcStandardJsonInputSettingsOptimizer;
use crate::solc::standard_json::input::settings::selection::Selection as SolcStandardJsonInputSettingsSelection;

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
    #[serde(skip_serializing)]
    pub suppressed_errors: Option<Vec<MessageType>>,
    /// The suppressed warnings.
    #[serde(skip_serializing)]
    pub suppressed_warnings: Option<Vec<MessageType>>,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    /// If the `path` is `None`, the input is read from the stdin.
    ///
    pub fn try_from(path: Option<&Path>) -> anyhow::Result<Self> {
        match path {
            Some(path) => {
                let file = std::fs::File::open(path).map_err(|error| {
                    anyhow::anyhow!("Standard JSON file {path:?} opening: {error}")
                })?;
                let input_json = std::io::read_to_string(file).map_err(|error| {
                    anyhow::anyhow!("Standard JSON file {path:?} reading: {error}")
                })?;
                let input: Self = era_compiler_common::deserialize_from_str(input_json.as_str())
                    .map_err(|error| {
                        anyhow::anyhow!("Standard JSON file {path:?} parsing: {error}")
                    })?;
                Ok(input)
            }
            None => {
                let input_json = std::io::read_to_string(std::io::stdin()).map_err(|error| {
                    anyhow::anyhow!("Standard JSON reading from stdin: {error}")
                })?;
                let input: Self = era_compiler_common::deserialize_from_str(input_json.as_str())
                    .map_err(|error| {
                        anyhow::anyhow!("Standard JSON parsing from stdin: {error}")
                    })?;
                Ok(input)
            }
        }
    }

    ///
    /// A shortcut constructor from Solidity source paths.
    ///
    pub fn try_from_solidity_paths(
        language: Language,
        evm_version: Option<era_compiler_common::EVMVersion>,
        paths: &[PathBuf],
        library_map: Vec<String>,
        remappings: Option<BTreeSet<String>>,
        output_selection: SolcStandardJsonInputSettingsSelection,
        optimizer: SolcStandardJsonInputSettingsOptimizer,
        metadata: Option<SolcStandardJsonInputSettingsMetadata>,
        force_evmla: bool,
        via_ir: bool,
        enable_eravm_extensions: bool,
        detect_missing_libraries: bool,
        llvm_options: Vec<String>,
        suppressed_errors: Vec<MessageType>,
        suppressed_warnings: Vec<MessageType>,
    ) -> anyhow::Result<Self> {
        let mut paths: BTreeSet<PathBuf> = paths.iter().cloned().collect();
        let libraries = Settings::parse_libraries(library_map)?;
        for library_file in libraries.keys() {
            paths.insert(PathBuf::from(library_file));
        }

        let sources = paths
            .into_par_iter()
            .map(|path| {
                let source = Source::try_read(path.as_path())?;
                Ok((path.to_string_lossy().to_string(), source))
            })
            .collect::<anyhow::Result<BTreeMap<String, Source>>>()?;

        Ok(Self {
            language,
            sources,
            settings: Settings::new(
                evm_version,
                libraries,
                remappings,
                output_selection,
                force_evmla,
                via_ir,
                enable_eravm_extensions,
                detect_missing_libraries,
                optimizer,
                llvm_options,
                metadata,
            ),
            suppressed_errors: Some(suppressed_errors),
            suppressed_warnings: Some(suppressed_warnings),
        })
    }

    ///
    /// A shortcut constructor from Solidity source code.
    ///
    pub fn try_from_solidity_sources(
        evm_version: Option<era_compiler_common::EVMVersion>,
        sources: BTreeMap<String, String>,
        libraries: BTreeMap<String, BTreeMap<String, String>>,
        remappings: Option<BTreeSet<String>>,
        output_selection: SolcStandardJsonInputSettingsSelection,
        optimizer: SolcStandardJsonInputSettingsOptimizer,
        metadata: Option<SolcStandardJsonInputSettingsMetadata>,
        force_evmla: bool,
        via_ir: bool,
        enable_eravm_extensions: bool,
        detect_missing_libraries: bool,
        llvm_options: Vec<String>,
        suppressed_errors: Vec<MessageType>,
        suppressed_warnings: Vec<MessageType>,
    ) -> anyhow::Result<Self> {
        let sources = sources
            .into_iter()
            .map(|(path, content)| (path, Source::from(content)))
            .collect();

        Ok(Self {
            language: Language::Solidity,
            sources,
            settings: Settings::new(
                evm_version,
                libraries,
                remappings,
                output_selection,
                force_evmla,
                via_ir,
                enable_eravm_extensions,
                detect_missing_libraries,
                optimizer,
                llvm_options,
                metadata,
            ),
            suppressed_errors: Some(suppressed_errors),
            suppressed_warnings: Some(suppressed_warnings),
        })
    }

    ///
    /// A shortcut constructor from source code.
    ///
    pub fn from_yul_sources(
        sources: BTreeMap<String, String>,
        libraries: BTreeMap<String, BTreeMap<String, String>>,
        optimizer: SolcStandardJsonInputSettingsOptimizer,
        llvm_options: Vec<String>,
    ) -> Self {
        let sources = sources
            .into_iter()
            .map(|(path, content)| (path, Source::from(content)))
            .collect();
        let output_selection = SolcStandardJsonInputSettingsSelection::new_yul_validation();

        Self {
            language: Language::Yul,
            sources,
            settings: Settings::new(
                None,
                libraries,
                None,
                output_selection,
                false,
                false,
                false,
                false,
                optimizer,
                llvm_options,
                None,
            ),
            suppressed_errors: None,
            suppressed_warnings: None,
        }
    }

    ///
    /// A shortcut constructor from source code.
    ///
    pub fn from_yul_paths(
        paths: &[PathBuf],
        libraries: BTreeMap<String, BTreeMap<String, String>>,
        optimizer: SolcStandardJsonInputSettingsOptimizer,
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
        let output_selection = SolcStandardJsonInputSettingsSelection::new_yul_validation();

        Self {
            language: Language::Yul,
            sources,
            settings: Settings::new(
                None,
                libraries,
                None,
                output_selection,
                false,
                false,
                false,
                false,
                optimizer,
                llvm_options,
                None,
            ),
            suppressed_errors: None,
            suppressed_warnings: None,
        }
    }

    ///
    /// Sets the necessary defaults for EraVM compilation.
    ///
    pub fn normalize(&mut self, version: &semver::Version, pipeline: Option<SolcPipeline>) {
        self.settings.normalize(version, pipeline);
    }

    ///
    /// Sets the necessary defaults for Yul validation.
    ///
    pub fn normalize_yul_validation(&mut self) {
        self.settings.normalize_yul_validation();
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
