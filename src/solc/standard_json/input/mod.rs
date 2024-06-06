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
use rayon::iter::ParallelIterator;
use serde::Deserialize;
use serde::Serialize;

use crate::solc::pipeline::Pipeline as SolcPipeline;
use crate::solc::standard_json::input::settings::metadata::Metadata as SolcStandardJsonInputSettingsMetadata;
use crate::solc::standard_json::input::settings::optimizer::Optimizer as SolcStandardJsonInputSettingsOptimizer;
use crate::solc::standard_json::input::settings::selection::Selection as SolcStandardJsonInputSettingsSelection;
use crate::warning::Warning;

use self::language::Language;
use self::settings::Settings;
use self::source::Source;

///
/// The `solc --standard-json` input.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    /// The input language.
    pub language: Language,
    /// The input source code files hashmap.
    pub sources: BTreeMap<String, Source>,
    /// The compiler settings.
    pub settings: Settings,

    /// The suppressed warnings.
    #[serde(skip_serializing)]
    pub suppressed_warnings: Option<Vec<Warning>>,
}

impl Input {
    ///
    /// A shortcut constructor.
    ///
    /// If the `path` is `None`, the input is read from the stdin.
    ///
    pub fn try_from_reader(path: Option<&Path>) -> anyhow::Result<Self> {
        match path {
            Some(path) => serde_json::from_reader(
                std::fs::File::open(path)
                    .map(std::io::BufReader::new)
                    .map_err(|error| {
                        anyhow::anyhow!("Standard JSON file {path:?} opening: {error}")
                    })?,
            ),
            None => serde_json::from_reader(std::io::BufReader::new(std::io::stdin())),
        }
        .map_err(|error| anyhow::anyhow!("Standard JSON reading: {error}"))
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
        via_ir: bool,
        suppressed_warnings: Option<Vec<Warning>>,
    ) -> anyhow::Result<Self> {
        let mut paths: BTreeSet<PathBuf> = paths.iter().cloned().collect();
        let libraries = Settings::parse_libraries(library_map)?;
        for library_file in libraries.keys() {
            paths.insert(PathBuf::from(library_file));
        }

        let sources = paths
            .into_par_iter()
            .map(|path| {
                let source = Source::try_read(path.as_path())
                    .unwrap_or_else(|error| panic!("Source code file {path:?} reading: {error}"));
                (path.to_string_lossy().to_string(), source)
            })
            .collect();

        Ok(Self {
            language,
            sources,
            settings: Settings::new(
                evm_version,
                libraries,
                remappings,
                output_selection,
                via_ir,
                optimizer,
                metadata,
            ),
            suppressed_warnings,
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
        via_ir: bool,
        suppressed_warnings: Option<Vec<Warning>>,
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
                via_ir,
                optimizer,
                metadata,
            ),
            suppressed_warnings,
        })
    }

    ///
    /// A shortcut constructor from source code.
    ///
    pub fn from_yul_sources(
        sources: BTreeMap<String, String>,
        libraries: BTreeMap<String, BTreeMap<String, String>>,
        optimizer: SolcStandardJsonInputSettingsOptimizer,
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
                optimizer,
                None,
            ),
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
                optimizer,
                None,
            ),
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
    /// Returns an owned tree of loaded sources.
    ///
    pub fn sources(&self) -> anyhow::Result<BTreeMap<String, String>> {
        self.sources
            .iter()
            .map(|(path, source)| {
                let source: String = source
                    .to_owned()
                    .try_into()
                    .map_err(|error| anyhow::anyhow!("Source `{path}`: {error}"))?;
                Ok((path.to_owned(), source))
            })
            .collect()
    }
}
