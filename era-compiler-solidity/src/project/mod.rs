//!
//! The project representation.
//!

pub mod contract;
pub mod thread_pool_evm;

use std::collections::BTreeMap;
use std::collections::HashSet;
use std::path::PathBuf;

use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use crate::build_eravm::contract::Contract as EraVMContractBuild;
use crate::build_eravm::Build as EraVMBuild;
use crate::build_evm::Build as EVMBuild;
use crate::evmla::assembly::Assembly;
use crate::missing_libraries::MissingLibraries;
use crate::process::input_eravm::Input as EraVMProcessInput;
use crate::process::input_evm::dependency_data::DependencyData as EVMProcessInputDependencyData;
use crate::process::input_evm::Input as EVMProcessInput;
use crate::process::output_eravm::Output as EraVMOutput;

use self::contract::factory_dependency::FactoryDependency;
use self::contract::ir::eravm_assembly::EraVMAssembly as ContractEraVMAssembly;
use self::contract::ir::evmla::EVMLA as ContractEVMLA;
use self::contract::ir::llvm_ir::LLVMIR as ContractLLVMIR;
use self::contract::ir::yul::Yul as ContractYul;
use self::contract::Contract;
use self::thread_pool_evm::ThreadPool as EVMThreadPool;

///
/// The project representation.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Project {
    /// The project language.
    pub language: era_solc::StandardJsonInputLanguage,
    /// The `solc` compiler version.
    pub solc_version: Option<era_solc::Version>,
    /// The project build results.
    pub contracts: BTreeMap<String, Contract>,
    /// The mapping of auxiliary identifiers, e.g. Yul object names, to full contract paths.
    pub identifier_paths: BTreeMap<String, String>,
    /// The library addresses.
    pub libraries: era_solc::StandardJsonInputLibraries,
}

impl Project {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        language: era_solc::StandardJsonInputLanguage,
        solc_version: Option<era_solc::Version>,
        contracts: BTreeMap<String, Contract>,
        libraries: era_solc::StandardJsonInputLibraries,
    ) -> Self {
        let mut identifier_paths = BTreeMap::new();
        for (path, contract) in contracts.iter() {
            identifier_paths.insert(contract.identifier().to_owned(), path.to_owned());
        }

        Self {
            language,
            solc_version,
            contracts,
            identifier_paths,
            libraries,
        }
    }

    ///
    /// Parses the Solidity `sources` and returns a Solidity project.
    ///
    pub fn try_from_solc_output(
        libraries: era_solc::StandardJsonInputLibraries,
        codegen: era_solc::StandardJsonInputCodegen,
        solc_output: &mut era_solc::StandardJsonOutput,
        solc_compiler: &era_solc::Compiler,
        debug_config: Option<&era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<Self> {
        if let era_solc::StandardJsonInputCodegen::EVMLA = codegen {
            Assembly::preprocess_dependencies(&mut solc_output.contracts)?;
        }

        let solc_version = solc_compiler.version.to_owned();

        let mut input_contracts = Vec::with_capacity(solc_output.contracts.len());
        for (path, file) in solc_output.contracts.iter() {
            for (name, contract) in file.iter() {
                input_contracts.push((path, name, contract));
            }
        }

        let results = input_contracts
            .par_iter()
            .map(
                |(path, name, contract): &(
                    &String,
                    &String,
                    &era_solc::StandardJsonOutputContract,
                )|
                 -> (String, anyhow::Result<Option<Contract>>) {
                    let name = era_compiler_common::ContractName::new(
                        (*path).to_owned(),
                        Some((*name).to_owned()),
                    );
                    let full_path = name.full_path.clone();

                    let result = match codegen {
                        era_solc::StandardJsonInputCodegen::Yul => ContractYul::try_from_source(
                            &name,
                            contract.ir_optimized.as_str(),
                            debug_config,
                        )
                        .map(|ir| ir.map(ContractYul::into)),
                        era_solc::StandardJsonInputCodegen::EVMLA => {
                            Ok(ContractEVMLA::try_from_contract(contract).map(ContractEVMLA::into))
                        }
                    }
                    .map(|source| {
                        source
                            .map(|source| Contract::new(name, source, contract.metadata.to_owned()))
                    });
                    (full_path, result)
                },
            )
            .collect::<BTreeMap<String, anyhow::Result<Option<Contract>>>>();

        let mut contracts = BTreeMap::new();
        for (path, result) in results.into_iter() {
            match result {
                Ok(Some(contract)) => {
                    contracts.insert(path, contract);
                }
                Ok(None) => continue,
                Err(error) => solc_output.push_error(Some(path), error),
            }
        }
        Ok(Project::new(
            era_solc::StandardJsonInputLanguage::Solidity,
            Some(solc_version),
            contracts,
            libraries,
        ))
    }

    ///
    /// Reads the Yul source code `paths` and returns a Yul project.
    ///
    pub fn try_from_yul_paths(
        paths: &[PathBuf],
        libraries: era_solc::StandardJsonInputLibraries,
        solc_output: Option<&mut era_solc::StandardJsonOutput>,
        solc_version: Option<&era_solc::Version>,
        debug_config: Option<&era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<Self> {
        let sources = paths
            .iter()
            .map(|path| {
                let source = era_solc::StandardJsonInputSource::from(path.as_path());
                (path.to_string_lossy().to_string(), source)
            })
            .collect::<BTreeMap<String, era_solc::StandardJsonInputSource>>();
        Self::try_from_yul_sources(sources, libraries, solc_output, solc_version, debug_config)
    }

    ///
    /// Parses the Yul `sources` and returns a Yul project.
    ///
    pub fn try_from_yul_sources(
        sources: BTreeMap<String, era_solc::StandardJsonInputSource>,
        libraries: era_solc::StandardJsonInputLibraries,
        mut solc_output: Option<&mut era_solc::StandardJsonOutput>,
        solc_version: Option<&era_solc::Version>,
        debug_config: Option<&era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<Self> {
        let results = sources
            .into_par_iter()
            .map(|(path, mut source)| {
                let name = era_compiler_common::ContractName::new(path.clone(), None);
                let source_code = match source.try_resolve() {
                    Ok(()) => source.take_content().expect("Always exists"),
                    Err(error) => return (path, Err(error)),
                };
                let source_hash = era_compiler_common::Hash::keccak256(source_code.as_bytes());

                let result =
                    ContractYul::try_from_source(&name, source_code.as_str(), debug_config).map(
                        |ir| {
                            ir.map(ContractYul::into).map(|ir| {
                                Contract::new(
                                    name,
                                    ir,
                                    serde_json::json!({
                                        "source_hash": source_hash.to_string(),
                                        "solc_version": solc_version,
                                    }),
                                )
                            })
                        },
                    );

                (path, result)
            })
            .collect::<BTreeMap<String, anyhow::Result<Option<Contract>>>>();

        let mut contracts = BTreeMap::new();
        for (path, result) in results.into_iter() {
            match result {
                Ok(Some(contract)) => {
                    contracts.insert(path, contract);
                }
                Ok(None) => continue,
                Err(error) => match solc_output {
                    Some(ref mut solc_output) => solc_output.push_error(Some(path), error),
                    None => anyhow::bail!(error),
                },
            }
        }
        Ok(Self::new(
            era_solc::StandardJsonInputLanguage::Yul,
            solc_version.cloned(),
            contracts,
            libraries,
        ))
    }

    ///
    /// Reads the LLVM IR source code `paths` and returns an LLVM IR project.
    ///
    pub fn try_from_llvm_ir_paths(
        paths: &[PathBuf],
        libraries: era_solc::StandardJsonInputLibraries,
        solc_output: Option<&mut era_solc::StandardJsonOutput>,
    ) -> anyhow::Result<Self> {
        let sources = paths
            .iter()
            .map(|path| {
                let source = era_solc::StandardJsonInputSource::from(path.as_path());
                (path.to_string_lossy().to_string(), source)
            })
            .collect::<BTreeMap<String, era_solc::StandardJsonInputSource>>();
        Self::try_from_llvm_ir_sources(sources, libraries, solc_output)
    }

    ///
    /// Parses the LLVM IR `sources` and returns an LLVM IR project.
    ///
    pub fn try_from_llvm_ir_sources(
        sources: BTreeMap<String, era_solc::StandardJsonInputSource>,
        libraries: era_solc::StandardJsonInputLibraries,
        mut solc_output: Option<&mut era_solc::StandardJsonOutput>,
    ) -> anyhow::Result<Self> {
        let results = sources
            .into_par_iter()
            .map(|(path, mut source)| {
                let source_code = match source.try_resolve() {
                    Ok(()) => source.take_content().expect("Always exists"),
                    Err(error) => return (path, Err(error)),
                };

                let source_hash = era_compiler_common::Hash::keccak256(source_code.as_bytes());

                let contract = Contract::new(
                    era_compiler_common::ContractName::new(path.clone(), None),
                    ContractLLVMIR::new(path.clone(), source_code).into(),
                    serde_json::json!({
                        "source_hash": source_hash.to_string(),
                    }),
                );

                (path, Ok(contract))
            })
            .collect::<BTreeMap<String, anyhow::Result<Contract>>>();

        let mut contracts = BTreeMap::new();
        for (path, result) in results.into_iter() {
            match result {
                Ok(contract) => {
                    contracts.insert(path, contract);
                }
                Err(error) => match solc_output {
                    Some(ref mut solc_output) => solc_output.push_error(Some(path), error),
                    None => anyhow::bail!(error),
                },
            }
        }
        Ok(Self::new(
            era_solc::StandardJsonInputLanguage::LLVMIR,
            None,
            contracts,
            libraries,
        ))
    }

    ///
    /// Reads the EraVM assembly source code `paths` and returns an EraVM assembly project.
    ///
    pub fn try_from_eravm_assembly_paths(
        paths: &[PathBuf],
        solc_output: Option<&mut era_solc::StandardJsonOutput>,
    ) -> anyhow::Result<Self> {
        let sources = paths
            .iter()
            .map(|path| {
                let source = era_solc::StandardJsonInputSource::from(path.as_path());
                (path.to_string_lossy().to_string(), source)
            })
            .collect::<BTreeMap<String, era_solc::StandardJsonInputSource>>();
        Self::try_from_eravm_assembly_sources(sources, solc_output)
    }

    ///
    /// Parses the EraVM assembly `sources` and returns an EraVM assembly project.
    ///
    pub fn try_from_eravm_assembly_sources(
        sources: BTreeMap<String, era_solc::StandardJsonInputSource>,
        mut solc_output: Option<&mut era_solc::StandardJsonOutput>,
    ) -> anyhow::Result<Self> {
        let results = sources
            .into_par_iter()
            .map(|(path, mut source)| {
                let source_code = match source.try_resolve() {
                    Ok(()) => source.take_content().expect("Always exists"),
                    Err(error) => return (path, Err(error)),
                };

                let source_hash = era_compiler_common::Hash::keccak256(source_code.as_bytes());

                let contract = Contract::new(
                    era_compiler_common::ContractName::new(path.clone(), None),
                    ContractEraVMAssembly::new(path.clone(), source_code).into(),
                    serde_json::json!({
                        "source_hash": source_hash.to_string(),
                    }),
                );

                (path, Ok(contract))
            })
            .collect::<BTreeMap<String, anyhow::Result<Contract>>>();

        let mut contracts = BTreeMap::new();
        for (path, result) in results.into_iter() {
            match result {
                Ok(contract) => {
                    contracts.insert(path, contract);
                }
                Err(error) => match solc_output {
                    Some(ref mut solc_output) => solc_output.push_error(Some(path), error),
                    None => anyhow::bail!(error),
                },
            }
        }
        Ok(Self::new(
            era_solc::StandardJsonInputLanguage::EraVMAssembly,
            None,
            contracts,
            era_solc::StandardJsonInputLibraries::default(),
        ))
    }

    ///
    /// Compiles all contracts to EraVM, returning their build artifacts.
    ///
    pub fn compile_to_eravm(
        self,
        messages: &mut Vec<era_solc::StandardJsonOutputError>,
        enable_eravm_extensions: bool,
        metadata_hash_type: era_compiler_common::HashType,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        output_assembly: bool,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<EraVMBuild> {
        let results = self.contracts.into_par_iter().map(|(path, mut contract)| {
            let factory_dependencies = contract
                .drain_factory_dependencies()
                .into_iter()
                .map(|identifier| self.identifier_paths.get(identifier.as_str()).cloned().expect("Always exists"))
                .collect();
            let input = EraVMProcessInput::new(
                contract,
                self.solc_version.clone(),
                self.identifier_paths.clone(),
                factory_dependencies,
                enable_eravm_extensions,
                metadata_hash_type,
                optimizer_settings.clone(),
                llvm_options.clone(),
                output_assembly,
                debug_config.clone(),
            );
            let result: crate::Result<EraVMOutput> =
                crate::process::call(path.as_str(), input, era_compiler_common::Target::EraVM);
            let result = result.map(|output| output.build);
            (path, result)
        }).collect::<BTreeMap<String, Result<EraVMContractBuild, era_solc::StandardJsonOutputError>>>();

        Ok(EraVMBuild::new(results, messages))
    }

    ///
    /// Compiles all contracts to EVM, returning their build artifacts.
    ///
    pub fn compile_to_evm(
        self,
        messages: &mut Vec<era_solc::StandardJsonOutputError>,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        metadata_hash_type: era_compiler_common::HashType,
        threads: Option<usize>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<EVMBuild> {
        let dependency_data =
            EVMProcessInputDependencyData::new(self.solc_version, self.identifier_paths);

        let input_template = EVMProcessInput::new(
            None,
            dependency_data,
            metadata_hash_type,
            optimizer_settings,
            llvm_options,
            debug_config,
        );
        let pool = EVMThreadPool::new(threads, self.contracts, input_template);
        pool.start();
        let results = pool.finish();
        Ok(EVMBuild::new(results, messages))
    }

    ///
    /// Get the list of missing deployable libraries.
    ///
    pub fn get_missing_libraries(&self) -> MissingLibraries {
        let deployed_libraries = self
            .libraries
            .as_inner()
            .iter()
            .flat_map(|(file, names)| {
                names
                    .iter()
                    .map(|(name, _address)| format!("{file}:{name}"))
                    .collect::<HashSet<String>>()
            })
            .collect::<HashSet<String>>();

        let mut missing_deployable_libraries = BTreeMap::new();
        for (contract_path, contract) in self.contracts.iter() {
            let missing_libraries = contract
                .get_missing_libraries()
                .into_iter()
                .filter(|library| !deployed_libraries.contains(library))
                .collect::<HashSet<String>>();
            missing_deployable_libraries.insert(contract_path.to_owned(), missing_libraries);
        }
        MissingLibraries::new(missing_deployable_libraries)
    }
}
