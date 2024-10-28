//!
//! The project representation.
//!

pub mod contract;
pub mod thread_pool_eravm;
pub mod thread_pool_evm;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;

use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use crate::build_eravm::Build as EraVMBuild;
use crate::build_evm::Build as EVMBuild;
use crate::process::input_eravm::dependency_data::DependencyData as EraVMProcessInputDependencyData;
use crate::process::input_eravm::Input as EraVMProcessInput;
use crate::process::input_evm::dependency_data::DependencyData as EVMProcessInputDependencyData;
use crate::process::input_evm::Input as EVMProcessInput;
use crate::solc::codegen::Codegen as SolcCodegen;
use crate::solc::standard_json::input::language::Language as SolcStandardJsonInputLanguage;
use crate::solc::standard_json::input::settings::libraries::missing::MissingLibraries;
use crate::solc::standard_json::input::settings::libraries::Libraries as SolcStandardJsonInputLibraries;
use crate::solc::standard_json::input::source::Source as SolcStandardJsonInputSource;
use crate::solc::standard_json::output::contract::Contract as SolcStandardJsonOutputContract;
use crate::solc::standard_json::output::error::Error as SolcStandardJsonOutputError;
use crate::solc::standard_json::output::Output as SolcStandardJsonOutput;
use crate::solc::version::Version as SolcVersion;
use crate::solc::Compiler as SolcCompiler;

use self::contract::ir::eravm_assembly::EraVMAssembly as ContractEraVMAssembly;
use self::contract::ir::evmla::EVMLA as ContractEVMLA;
use self::contract::ir::llvm_ir::LLVMIR as ContractLLVMIR;
use self::contract::ir::yul::Yul as ContractYul;
use self::contract::Contract;
use self::thread_pool_eravm::ThreadPool as EraVMThreadPool;
use self::thread_pool_evm::ThreadPool as EVMThreadPool;

///
/// The project representation.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Project {
    /// The project language.
    pub language: SolcStandardJsonInputLanguage,
    /// The `solc` compiler version.
    pub solc_version: Option<SolcVersion>,
    /// The project build results.
    pub contracts: BTreeMap<String, Contract>,
    /// The mapping of auxiliary identifiers, e.g. Yul object names, to full contract paths.
    pub identifier_paths: BTreeMap<String, String>,
    /// The library addresses.
    pub libraries: SolcStandardJsonInputLibraries,
}

impl Project {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        language: SolcStandardJsonInputLanguage,
        solc_version: Option<SolcVersion>,
        contracts: BTreeMap<String, Contract>,
        libraries: SolcStandardJsonInputLibraries,
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
        libraries: SolcStandardJsonInputLibraries,
        pipeline: SolcCodegen,
        solc_output: &mut SolcStandardJsonOutput,
        solc_compiler: &SolcCompiler,
        debug_config: Option<&era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<Self> {
        if let SolcCodegen::EVMLA = pipeline {
            solc_output.preprocess_dependencies()?;
        }

        let solc_version = solc_compiler.version.to_owned();

        let files = solc_output
            .contracts
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No input sources specified."))?;
        let mut input_contracts = Vec::with_capacity(files.len());
        for (path, contracts) in files.iter() {
            for (name, contract) in contracts.iter() {
                input_contracts.push((path, name, contract));
            }
        }

        let results =
            input_contracts
                .par_iter()
                .map(
                    |(path, name, contract): &(
                        &String,
                        &String,
                        &SolcStandardJsonOutputContract,
                    )|
                     -> (String, anyhow::Result<Option<Contract>>) {
                        let name = era_compiler_common::ContractName::new(
                            (*path).to_owned(),
                            Some((*name).to_owned()),
                        );
                        let full_path = name.full_path.clone();

                        let result = match pipeline {
                            SolcCodegen::Yul => ContractYul::try_from_source(
                                &name,
                                contract.ir_optimized.as_deref(),
                                debug_config,
                            )
                            .map(|ir| ir.map(ContractYul::into)),
                            SolcCodegen::EVMLA => {
                                Ok(ContractEVMLA::try_from_contract(contract)
                                    .map(ContractEVMLA::into))
                            }
                        }
                        .map(|source| {
                            source.map(|source| {
                                Contract::new(
                                    name,
                                    source,
                                    contract.metadata.to_owned().expect("Always exists"),
                                )
                            })
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
            SolcStandardJsonInputLanguage::Solidity,
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
        libraries: SolcStandardJsonInputLibraries,
        solc_output: Option<&mut SolcStandardJsonOutput>,
        solc_version: Option<&SolcVersion>,
        debug_config: Option<&era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<Self> {
        let sources = paths
            .iter()
            .map(|path| {
                let source = SolcStandardJsonInputSource::from(path.as_path());
                (path.to_string_lossy().to_string(), source)
            })
            .collect::<BTreeMap<String, SolcStandardJsonInputSource>>();
        Self::try_from_yul_sources(sources, libraries, solc_output, solc_version, debug_config)
    }

    ///
    /// Parses the Yul `sources` and returns a Yul project.
    ///
    pub fn try_from_yul_sources(
        sources: BTreeMap<String, SolcStandardJsonInputSource>,
        libraries: SolcStandardJsonInputLibraries,
        mut solc_output: Option<&mut SolcStandardJsonOutput>,
        solc_version: Option<&SolcVersion>,
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
                    ContractYul::try_from_source(&name, Some(source_code.as_str()), debug_config)
                        .map(|ir| {
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
                        });

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
            SolcStandardJsonInputLanguage::Yul,
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
        libraries: SolcStandardJsonInputLibraries,
        solc_output: Option<&mut SolcStandardJsonOutput>,
    ) -> anyhow::Result<Self> {
        let sources = paths
            .iter()
            .map(|path| {
                let source = SolcStandardJsonInputSource::from(path.as_path());
                (path.to_string_lossy().to_string(), source)
            })
            .collect::<BTreeMap<String, SolcStandardJsonInputSource>>();
        Self::try_from_llvm_ir_sources(sources, libraries, solc_output)
    }

    ///
    /// Parses the LLVM IR `sources` and returns an LLVM IR project.
    ///
    pub fn try_from_llvm_ir_sources(
        sources: BTreeMap<String, SolcStandardJsonInputSource>,
        libraries: SolcStandardJsonInputLibraries,
        mut solc_output: Option<&mut SolcStandardJsonOutput>,
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
            SolcStandardJsonInputLanguage::LLVMIR,
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
        solc_output: Option<&mut SolcStandardJsonOutput>,
    ) -> anyhow::Result<Self> {
        let sources = paths
            .iter()
            .map(|path| {
                let source = SolcStandardJsonInputSource::from(path.as_path());
                (path.to_string_lossy().to_string(), source)
            })
            .collect::<BTreeMap<String, SolcStandardJsonInputSource>>();
        Self::try_from_eravm_assembly_sources(sources, solc_output)
    }

    ///
    /// Parses the EraVM assembly `sources` and returns an EraVM assembly project.
    ///
    pub fn try_from_eravm_assembly_sources(
        sources: BTreeMap<String, SolcStandardJsonInputSource>,
        mut solc_output: Option<&mut SolcStandardJsonOutput>,
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
            SolcStandardJsonInputLanguage::EraVMAssembly,
            None,
            contracts,
            SolcStandardJsonInputLibraries::default(),
        ))
    }

    ///
    /// Compiles all contracts to EraVM, returning their build artifacts.
    ///
    pub fn compile_to_eravm(
        self,
        messages: &mut Vec<SolcStandardJsonOutputError>,
        enable_eravm_extensions: bool,
        linker_symbols: BTreeMap<String, [u8; era_compiler_common::BYTE_LENGTH_ETH_ADDRESS]>,
        metadata_hash_type: era_compiler_common::HashType,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        output_assembly: bool,
        threads: Option<usize>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<EraVMBuild> {
        let identifier_paths = self.identifier_paths.clone();
        let dependency_data =
            EraVMProcessInputDependencyData::new(self.solc_version, self.identifier_paths.clone());

        let input_template = EraVMProcessInput::new(
            None,
            dependency_data,
            enable_eravm_extensions,
            linker_symbols,
            metadata_hash_type,
            optimizer_settings,
            llvm_options,
            output_assembly,
            debug_config,
        );
        let pool = EraVMThreadPool::new(threads, self.contracts, input_template);
        pool.start();
        let results = pool.finish();

        let mut hashes = HashMap::with_capacity(results.len());
        for (path, result) in results.iter() {
            if let Some(bytecode_hash) = result
                .as_ref()
                .ok()
                .and_then(|contract| contract.build.bytecode_hash)
            {
                hashes.insert(path.to_owned(), bytecode_hash.to_owned());
            }
        }

        let results = results
            .into_iter()
            .map(|(path, mut result)| {
                if let Ok(ref mut contract) = result {
                    for dependency in contract.factory_dependencies.drain() {
                        let dependency_path = identifier_paths
                            .get(dependency.as_str())
                            .cloned()
                            .unwrap_or_else(|| {
                                panic!("dependency `{dependency}` full path not found")
                            });
                        if let Some(hash) = hashes.get(dependency_path.as_str()) {
                            contract
                                .build
                                .factory_dependencies
                                .insert(hex::encode(hash), dependency_path);
                        }
                    }
                }
                (path, result)
            })
            .collect();
        Ok(EraVMBuild::new(results, messages))
    }

    ///
    /// Compiles all contracts to EVM, returning their build artifacts.
    ///
    pub fn compile_to_evm(
        self,
        messages: &mut Vec<SolcStandardJsonOutputError>,
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
