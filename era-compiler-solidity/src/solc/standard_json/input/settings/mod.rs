//!
//! The `solc --standard-json` input settings.
//!

pub mod metadata;
pub mod optimizer;
pub mod selection;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashSet;

use crate::solc::pipeline::Pipeline as SolcPipeline;

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
    /// The target EVM version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evm_version: Option<era_compiler_common::EVMVersion>,
    /// The linker library addresses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub libraries: Option<BTreeMap<String, BTreeMap<String, String>>>,
    /// The sorted list of remappings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remappings: Option<BTreeSet<String>>,
    /// The output selection filters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_selection: Option<Selection>,
    /// Whether to compile via EVM assembly.
    #[serde(rename = "forceEVMLA", skip_serializing)]
    pub force_evmla: Option<bool>,
    /// Whether to add the Yul step to compilation via EVM assembly.
    #[serde(
        rename = "viaIR",
        skip_deserializing,
        skip_serializing_if = "Option::is_none"
    )]
    pub via_ir: Option<bool>,
    /// Whether to enable EraVM extensions.
    #[serde(rename = "enableEraVMExtensions", skip_serializing)]
    pub enable_eravm_extensions: Option<bool>,
    /// Whether to enable the missing libraries detection mode.
    #[serde(rename = "detectMissingLibraries", skip_serializing)]
    pub detect_missing_libraries: Option<bool>,
    /// The optimizer settings.
    pub optimizer: Optimizer,
    /// The extra LLVM options.
    #[serde(rename = "LLVMOptions", skip_serializing)]
    pub llvm_options: Option<Vec<String>>,
    /// The metadata settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl Settings {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        evm_version: Option<era_compiler_common::EVMVersion>,
        libraries: BTreeMap<String, BTreeMap<String, String>>,
        remappings: Option<BTreeSet<String>>,
        output_selection: Selection,
        force_evmla: bool,
        via_ir: bool,
        enable_eravm_extensions: bool,
        detect_missing_libraries: bool,
        optimizer: Optimizer,
        llvm_options: Vec<String>,
        metadata: Option<Metadata>,
    ) -> Self {
        Self {
            evm_version,
            libraries: Some(libraries),
            remappings,
            output_selection: Some(output_selection),
            force_evmla: if force_evmla { Some(true) } else { None },
            via_ir: if via_ir { Some(true) } else { None },
            enable_eravm_extensions: if enable_eravm_extensions {
                Some(true)
            } else {
                None
            },
            detect_missing_libraries: if detect_missing_libraries {
                Some(true)
            } else {
                None
            },
            optimizer,
            llvm_options: Some(llvm_options),
            metadata,
        }
    }

    ///
    /// Sets the necessary defaults for EraVM compilation.
    ///
    pub fn normalize(&mut self, version: &semver::Version, pipeline: Option<SolcPipeline>) {
        self.output_selection
            .get_or_insert_with(Selection::default)
            .extend_with_required(pipeline);

        self.optimizer.normalize(version);
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

    ///
    /// Parses the library list and returns their double hashmap with path and name as keys.
    ///
    pub fn parse_libraries(
        input: Vec<String>,
    ) -> anyhow::Result<BTreeMap<String, BTreeMap<String, String>>> {
        let mut libraries = BTreeMap::new();
        for (index, library) in input.into_iter().enumerate() {
            let mut path_and_address = library.split('=');
            let path = path_and_address
                .next()
                .ok_or_else(|| anyhow::anyhow!("The library #{} path is missing", index))?;
            let mut file_and_contract = path.split(':');
            let file = file_and_contract
                .next()
                .ok_or_else(|| anyhow::anyhow!("The library `{}` file name is missing", path))?;
            let contract = file_and_contract.next().ok_or_else(|| {
                anyhow::anyhow!("The library `{}` contract name is missing", path)
            })?;
            let address = path_and_address
                .next()
                .ok_or_else(|| anyhow::anyhow!("The library `{}` address is missing", path))?;
            libraries
                .entry(file.to_owned())
                .or_insert_with(BTreeMap::new)
                .insert(contract.to_owned(), address.to_owned());
        }
        Ok(libraries)
    }
}
