//!
//! The `solc --standard-json` output.
//!

pub mod contract;
pub mod error;
pub mod source;

use std::collections::BTreeMap;
use std::collections::HashSet;
use std::io::Write;

use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use crate::evmla::assembly::instruction::Instruction;
use crate::evmla::assembly::Assembly;
use crate::solc::pipeline::Pipeline as SolcPipeline;
use crate::solc::standard_json::input::settings::selection::file::flag::Flag as SelectionFlag;
use crate::solc::standard_json::output::contract::evm::EVM as StandardJSONOutputContractEVM;
use crate::solc::version::Version as SolcVersion;
use crate::warning::Warning;

use self::contract::Contract;
use self::error::source_location::SourceLocation as JsonOutputErrorSourceLocation;
use self::error::Error as JsonOutputError;
use self::source::Source;

///
/// The `solc --standard-json` output.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Output {
    /// The file-contract hashmap.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contracts: Option<BTreeMap<String, BTreeMap<String, Contract>>>,
    /// The source code mapping data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sources: Option<BTreeMap<String, Source>>,
    /// The compilation errors and warnings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<JsonOutputError>>,

    /// The `solc` compiler version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The `solc` compiler long version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_version: Option<String>,
    /// The `zksolc` compiler version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zk_version: Option<String>,
}

impl Output {
    ///
    /// Initializes a standard JSON output.
    ///
    /// Is used for projects compiled without `solc`.
    ///
    pub fn new(sources: &BTreeMap<String, String>, messages: Vec<JsonOutputError>) -> Self {
        let contracts = sources
            .keys()
            .map(|path| {
                let mut contracts = BTreeMap::new();
                contracts.insert(path.to_owned(), Contract::default());
                (path.to_owned(), contracts)
            })
            .collect::<BTreeMap<String, BTreeMap<String, Contract>>>();

        let sources = sources
            .keys()
            .enumerate()
            .map(|(index, path)| (path.to_owned(), Source::new(index)))
            .collect::<BTreeMap<String, Source>>();

        Self {
            contracts: Some(contracts),
            sources: Some(sources),
            errors: Some(messages),
            version: None,
            long_version: None,
            zk_version: Some(env!("CARGO_PKG_VERSION").to_owned()),
        }
    }

    ///
    /// Prunes the output JSON and prints it to stdout.
    ///
    pub fn write_and_exit(mut self, prune_output: HashSet<SelectionFlag>) -> ! {
        let sources = self
            .sources
            .as_mut()
            .map(|sources| sources.values_mut().collect::<Vec<&mut Source>>())
            .unwrap_or_default();
        for source in sources.into_iter() {
            if prune_output.contains(&SelectionFlag::AST) {
                source.ast = None;
            }
        }

        let contracts = self
            .contracts
            .as_mut()
            .map(|contracts| {
                contracts
                    .values_mut()
                    .flat_map(|contracts| contracts.values_mut())
                    .collect::<Vec<&mut Contract>>()
            })
            .unwrap_or_default();
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

        if let Some(ref mut files) = self.contracts {
            files.retain(|_, contracts| {
                contracts.retain(|_, contract| !contract.is_empty());
                !contracts.is_empty()
            });
        }

        if let Some(ref mut errors) = self.errors {
            errors.retain(|error| {
                JsonOutputError::IGNORED_WARNING_CODES
                    .into_iter()
                    .map(Some)
                    .all(|code| code != error.error_code.as_deref())
            });
        }

        serde_json::to_writer(std::io::stdout(), &self).expect("Stdout writing error");
        std::process::exit(era_compiler_common::EXIT_CODE_SUCCESS);
    }

    ///
    /// Removes EVM artifacts to prevent their accidental usage.
    ///
    pub fn remove_evm(&mut self) {
        if let Some(files) = self.contracts.as_mut() {
            for (_, file) in files.iter_mut() {
                for (_, contract) in file.iter_mut() {
                    if let Some(evm) = contract.evm.as_mut() {
                        evm.bytecode = None;
                    }
                }
            }
        }
    }

    ///
    /// Pushes an arbitrary error with path.
    ///
    /// Please do not push project-general errors without paths here.
    ///
    pub fn push_error(&mut self, path: String, error: anyhow::Error) {
        self.errors
            .get_or_insert_with(Vec::new)
            .push(JsonOutputError::new_error(
                error,
                Some(JsonOutputErrorSourceLocation::new(path)),
            ));
    }

    ///
    /// Traverses the AST and returns the list of additional errors and warnings.
    ///
    pub fn preprocess_ast(
        &mut self,
        version: &SolcVersion,
        pipeline: SolcPipeline,
        suppressed_warnings: &[Warning],
    ) -> anyhow::Result<()> {
        let sources = match self.sources.as_ref() {
            Some(sources) => sources,
            None => return Ok(()),
        };
        let id_paths: BTreeMap<usize, &String> = sources
            .iter()
            .map(|(path, source)| (source.id, path))
            .collect();

        let messages: Vec<JsonOutputError> = sources
            .par_iter()
            .map(|(_path, source)| {
                source
                    .ast
                    .as_ref()
                    .map(|ast| {
                        Source::get_messages(ast, &id_paths, version, pipeline, suppressed_warnings)
                    })
                    .unwrap_or_default()
            })
            .flatten()
            .collect();
        self.errors.get_or_insert_with(Vec::new).extend(messages);

        Ok(())
    }

    ///
    /// The pass, which replaces with dependency indexes with actual data.
    ///
    pub fn preprocess_dependencies(&mut self) -> anyhow::Result<()> {
        let files = match self.contracts.as_mut() {
            Some(files) => files,
            None => return Ok(()),
        };
        let mut hash_path_mapping = BTreeMap::new();

        for (path, contracts) in files.iter() {
            for (name, contract) in contracts.iter() {
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
        for (path, contracts) in files.iter_mut() {
            for (name, contract) in contracts.iter_mut() {
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
    /// Checks for errors, returning `Err` if there is at least one error.
    ///
    /// Removes warnings from the list of errors and prints them to stderr.
    ///
    pub fn handle_errors(&mut self) -> anyhow::Result<()> {
        let errors = match self.errors.as_mut() {
            Some(errors) => errors,
            None => return Ok(()),
        };
        if errors.iter().any(|error| error.severity == "error") {
            anyhow::bail!(
                "{}",
                errors
                    .iter()
                    .map(|error| error.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            );
        } else {
            writeln!(
                std::io::stderr(),
                "{}",
                errors
                    .drain(..)
                    .map(|error| error.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            )
            .expect("Stderr writing error");
        }

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
