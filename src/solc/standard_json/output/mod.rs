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

use crate::evmla::assembly::instruction::Instruction;
use crate::evmla::assembly::Assembly;
use crate::solc::pipeline::Pipeline as SolcPipeline;
use crate::solc::standard_json::input::settings::selection::file::flag::Flag as SelectionFlag;
use crate::solc::version::Version as SolcVersion;
use crate::warning::Warning;

use self::contract::Contract;
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
    pub fn new(sources: &BTreeMap<String, String>) -> Self {
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
            errors: None,
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
            if let Some(ref mut evm) = contract.evm {
                if prune_output.contains(&SelectionFlag::EVMLA) {
                    evm.assembly = None;
                }
                if prune_output.contains(&SelectionFlag::MethodIdentifiers) {
                    evm.method_identifiers = None;
                }
            }
            if prune_output.contains(&SelectionFlag::Yul) {
                contract.ir_optimized = None;
            }
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
        let mut error = JsonOutputError {
            component: "general".to_owned(),
            error_code: None,
            formatted_message: error.to_string(),
            message: "".to_owned(),
            severity: "error".to_owned(),
            source_location: None,
            r#type: "Error".to_owned(),
        };
        error.push_contract_path(path.as_str());

        self.errors.get_or_insert_with(Vec::new).push(error);
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

        let messages: Vec<JsonOutputError> = sources
            .par_iter()
            .map(|(path, source)| {
                if let Some(ast) = source.ast.as_ref() {
                    let mut eravm_messages =
                        Source::get_messages(ast, version, pipeline, suppressed_warnings);
                    for message in eravm_messages.iter_mut() {
                        message.push_contract_path(path.as_str());
                    }
                    eravm_messages
                } else {
                    vec![]
                }
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
                    .and_then(|evm| evm.assembly.as_ref())
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
                let assembly = match contract.evm.as_mut().and_then(|evm| evm.assembly.as_mut()) {
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
    pub fn check_errors(&self) -> anyhow::Result<()> {
        if self
            .errors
            .as_ref()
            .map(|errors| errors.iter().any(|error| error.severity == "error"))
            .unwrap_or_default()
        {
            anyhow::bail!(
                "{}",
                self.errors
                    .as_ref()
                    .map(|errors| {
                        errors
                            .iter()
                            .map(|error| error.to_string())
                            .collect::<Vec<String>>()
                            .join("\n")
                    })
                    .unwrap_or_default()
            );
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
