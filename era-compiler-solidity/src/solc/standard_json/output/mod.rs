//!
//! The `solc --standard-json` output.
//!

pub mod contract;
pub mod error;
pub mod source;

use std::collections::BTreeMap;
use std::collections::HashSet;

use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use crate::error_type::ErrorType;
use crate::evmla::assembly::instruction::Instruction;
use crate::evmla::assembly::Assembly;
use crate::solc::codegen::Codegen as SolcCodegen;
use crate::solc::standard_json::input::settings::selection::file::flag::Flag as SelectionFlag;
use crate::solc::standard_json::input::source::Source as StandardJSONInputSource;
use crate::solc::standard_json::output::contract::evm::EVM as StandardJSONOutputContractEVM;
use crate::solc::version::Version as SolcVersion;
use crate::warning_type::WarningType;

use self::contract::Contract;
use self::error::collectable::Collectable as CollectableError;
use self::error::source_location::SourceLocation as JsonOutputErrorSourceLocation;
use self::error::Error as JsonOutputError;
use self::source::Source;

///
/// The `solc --standard-json` output.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Output {
    /// The file-contract hashmap.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub contracts: BTreeMap<String, BTreeMap<String, Contract>>,
    /// The source code mapping data.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub sources: BTreeMap<String, Source>,
    /// The compilation errors and warnings.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<JsonOutputError>,

    /// The `solc` compiler version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The `solc` compiler long version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_version: Option<String>,
    /// The `zksolc` compiler version.
    #[serde(default = "crate::version")]
    pub zk_version: String,
}

impl Output {
    ///
    /// Initializes a standard JSON output.
    ///
    /// Is used for projects compiled without `solc`.
    ///
    pub fn new(
        sources: &BTreeMap<String, StandardJSONInputSource>,
        messages: &mut Vec<JsonOutputError>,
    ) -> Self {
        let sources = sources
            .keys()
            .enumerate()
            .map(|(index, path)| (path.to_owned(), Source::new(index)))
            .collect::<BTreeMap<String, Source>>();

        Self {
            contracts: BTreeMap::new(),
            sources,
            errors: std::mem::take(messages),

            version: None,
            long_version: None,
            zk_version: crate::version(),
        }
    }

    ///
    /// Initializes a standard JSON output with messages.
    ///
    /// Is used to emit errors in standard JSON mode.
    ///
    pub fn new_with_messages(messages: Vec<JsonOutputError>) -> Self {
        Self {
            contracts: BTreeMap::new(),
            sources: BTreeMap::new(),
            errors: messages,

            version: None,
            long_version: None,
            zk_version: crate::version(),
        }
    }

    ///
    /// Prunes the output JSON and prints it to stdout.
    ///
    pub fn write_and_exit(mut self, prune_output: HashSet<SelectionFlag>) -> ! {
        let sources = self.sources.values_mut().collect::<Vec<&mut Source>>();
        for source in sources.into_iter() {
            if prune_output.contains(&SelectionFlag::AST) {
                source.ast = None;
            }
        }

        let contracts = self
            .contracts
            .values_mut()
            .flat_map(|contracts| contracts.values_mut())
            .collect::<Vec<&mut Contract>>();
        for contract in contracts.into_iter() {
            if prune_output.contains(&SelectionFlag::Metadata) {
                contract.metadata = None;
            }
            if prune_output.contains(&SelectionFlag::Yul) {
                contract.ir_optimized = None;
            }
            if let Some(ref mut evm) = contract.evm {
                if prune_output.contains(&SelectionFlag::EVMLA) {
                    evm.legacy_assembly = None;
                }
                if prune_output.contains(&SelectionFlag::MethodIdentifiers) {
                    evm.method_identifiers = None;
                }
                evm.extra_metadata = None;
            }
            if contract
                .evm
                .as_ref()
                .map(StandardJSONOutputContractEVM::is_empty)
                .unwrap_or_default()
            {
                contract.evm = None;
            }
        }

        self.contracts.retain(|_, contracts| {
            contracts.retain(|_, contract| !contract.is_empty());
            !contracts.is_empty()
        });

        serde_json::to_writer(std::io::stdout(), &self).expect("Stdout writing error");
        std::process::exit(era_compiler_common::EXIT_CODE_SUCCESS);
    }

    ///
    /// Removes EVM artifacts to prevent their accidental usage.
    ///
    pub fn remove_evm(&mut self) {
        for (_, file) in self.contracts.iter_mut() {
            for (_, contract) in file.iter_mut() {
                if let Some(evm) = contract.evm.as_mut() {
                    evm.bytecode = None;
                }
            }
        }
    }

    ///
    /// Pushes an arbitrary error with path.
    ///
    /// Please do not push project-general errors without paths here.
    ///
    pub fn push_error(&mut self, path: Option<String>, error: anyhow::Error) {
        self.errors.push(JsonOutputError::new_error(
            error,
            path.map(JsonOutputErrorSourceLocation::new),
            None,
        ));
    }

    ///
    /// Traverses the AST and returns the list of additional errors and warnings.
    ///
    pub fn preprocess_ast(
        &mut self,
        sources: &BTreeMap<String, StandardJSONInputSource>,
        version: &SolcVersion,
        codegen: SolcCodegen,
        suppressed_errors: &[ErrorType],
        suppressed_warnings: &[WarningType],
    ) -> anyhow::Result<()> {
        let id_paths: BTreeMap<usize, &String> = self
            .sources
            .iter()
            .map(|(path, source)| (source.id, path))
            .collect();

        let messages: Vec<JsonOutputError> = self
            .sources
            .par_iter()
            .map(|(_path, source)| {
                source
                    .ast
                    .as_ref()
                    .map(|ast| {
                        Source::get_messages(
                            ast,
                            &id_paths,
                            sources,
                            version,
                            codegen,
                            suppressed_errors,
                            suppressed_warnings,
                        )
                    })
                    .unwrap_or_default()
            })
            .flatten()
            .collect();
        self.errors.extend(messages);

        Ok(())
    }

    ///
    /// The pass, which replaces with dependency indexes with actual data.
    ///
    pub fn preprocess_dependencies(&mut self) -> anyhow::Result<()> {
        let mut hash_path_mapping = BTreeMap::new();

        for (path, file) in self.contracts.iter() {
            for (name, contract) in file.iter() {
                let full_path = format!("{path}:{name}");
                let hash = match contract
                    .evm
                    .as_ref()
                    .and_then(|evm| evm.legacy_assembly.as_ref())
                    .map(|assembly| assembly.keccak256())
                {
                    Some(hash) => hash,
                    None => continue,
                };

                hash_path_mapping.insert(hash, full_path);
            }
        }

        let mut assemblies = BTreeMap::new();
        for (path, file) in self.contracts.iter_mut() {
            for (name, contract) in file.iter_mut() {
                let full_path = format!("{path}:{name}");
                let assembly = match contract
                    .evm
                    .as_mut()
                    .and_then(|evm| evm.legacy_assembly.as_mut())
                {
                    Some(assembly) => assembly,
                    None => continue,
                };
                assemblies.insert(full_path, assembly);
            }
        }
        assemblies
            .into_par_iter()
            .map(|(full_path, assembly)| {
                Self::preprocess_dependency_level(full_path.as_str(), assembly, &hash_path_mapping)
            })
            .collect::<anyhow::Result<()>>()?;

        Ok(())
    }

    ///
    /// Preprocesses an assembly JSON structure dependency data map.
    ///
    fn preprocess_dependency_level(
        full_path: &str,
        assembly: &mut Assembly,
        hash_path_mapping: &BTreeMap<String, String>,
    ) -> anyhow::Result<()> {
        assembly.set_full_path(full_path.to_owned());

        let deploy_code_index_path_mapping =
            assembly.deploy_dependencies_pass(full_path, hash_path_mapping)?;
        if let Some(deploy_code_instructions) = assembly.code.as_deref_mut() {
            Instruction::replace_data_aliases(
                deploy_code_instructions,
                &deploy_code_index_path_mapping,
            )?;
        };

        let runtime_code_index_path_mapping =
            assembly.runtime_dependencies_pass(full_path, hash_path_mapping)?;
        if let Some(runtime_code_instructions) = assembly
            .data
            .as_mut()
            .and_then(|data_map| data_map.get_mut("0"))
            .and_then(|data| data.get_assembly_mut())
            .and_then(|assembly| assembly.code.as_deref_mut())
        {
            Instruction::replace_data_aliases(
                runtime_code_instructions,
                &runtime_code_index_path_mapping,
            )?;
        }

        Ok(())
    }
}

impl CollectableError for Output {
    fn errors(&self) -> Vec<&JsonOutputError> {
        self.errors
            .iter()
            .filter(|error| error.severity == "error")
            .collect()
    }

    fn warnings(&self) -> Vec<&JsonOutputError> {
        self.errors
            .iter()
            .filter(|error| error.severity == "warning")
            .collect()
    }

    fn remove_warnings(&mut self) {
        self.errors.retain(|error| error.severity != "warning");
    }
}
