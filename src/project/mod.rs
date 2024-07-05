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
use sha3::Digest;

use crate::build_eravm::Build as EraVMBuild;
use crate::build_evm::Build as EVMBuild;
use crate::missing_libraries::MissingLibraries;
use crate::process::input_eravm::dependency_data::DependencyData as EraVMProcessInputDependencyData;
use crate::process::input_eravm::Input as EraVMProcessInput;
use crate::process::input_evm::dependency_data::DependencyData as EVMProcessInputDependencyData;
use crate::process::input_evm::Input as EVMProcessInput;
use crate::solc::pipeline::Pipeline as SolcPipeline;
use crate::solc::standard_json::input::language::Language as SolcStandardJsonInputLanguage;
use crate::solc::standard_json::input::source::Source as SolcStandardJsonInputSource;
use crate::solc::standard_json::output::error::Error as SolcStandardJsonOutputError;
use crate::solc::standard_json::output::Output as SolcStandardJsonOutput;
use crate::solc::version::Version as SolcVersion;
use crate::solc::Compiler as SolcCompiler;
use crate::yul::lexer::Lexer;
use crate::yul::parser::statement::object::Object;

use self::contract::ir::IR as ContractIR;
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
    pub libraries: BTreeMap<String, BTreeMap<String, String>>,
}

impl Project {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        language: SolcStandardJsonInputLanguage,
        solc_version: Option<SolcVersion>,
        contracts: BTreeMap<String, Contract>,
        libraries: BTreeMap<String, BTreeMap<String, String>>,
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
        libraries: BTreeMap<String, BTreeMap<String, String>>,
        pipeline: SolcPipeline,
        solc_output: &mut SolcStandardJsonOutput,
        solc_compiler: &SolcCompiler,
        debug_config: Option<&era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<Self> {
        if let SolcPipeline::EVMLA = pipeline {
            solc_output.preprocess_dependencies()?;
        }

        let solc_version = solc_compiler.version.to_owned();
        let files = match solc_output.contracts.as_ref() {
            Some(files) => files,
            None => anyhow::bail!("The project is empty"),
        };

        let mut input_contracts = Vec::with_capacity(files.len());
        for (path, contracts) in files.iter() {
            for (name, contract) in contracts.iter() {
                input_contracts.push((path, name, contract));
            }
        }

        let results = input_contracts
            .par_iter()
            .filter_map(
                |(path, name, contract)| -> Option<(String, anyhow::Result<Contract>)> {
                    let full_path = format!("{path}:{name}");

                    let source = match pipeline {
                        SolcPipeline::Yul => {
                            let ir_optimized = match contract.ir_optimized.to_owned() {
                                Some(ir_optimized) => ir_optimized,
                                None => return None,
                            };
                            if ir_optimized.is_empty() {
                                return None;
                            }

                            if let Some(debug_config) = debug_config {
                                if let Err(error) = debug_config.dump_yul(
                                    full_path.as_str(),
                                    None,
                                    ir_optimized.as_str(),
                                ) {
                                    return Some((full_path, Err(error)));
                                }
                            }

                            let mut lexer = Lexer::new(ir_optimized.to_owned());
                            let object = match Object::parse(&mut lexer, None)
                                .map_err(|error| anyhow::anyhow!("Yul parsing: {error:?}"))
                            {
                                Ok(object) => object,
                                Err(error) => return Some((full_path, Err(error))),
                            };

                            ContractIR::new_yul(object)
                        }
                        SolcPipeline::EVMLA => {
                            let evm = contract.evm.as_ref();
                            let assembly = match evm.and_then(|evm| evm.legacy_assembly.to_owned())
                            {
                                Some(assembly) => assembly.to_owned(),
                                None => return None,
                            };
                            let extra_metadata = evm
                                .and_then(|evm| evm.extra_metadata.to_owned())
                                .unwrap_or_default();

                            ContractIR::new_evmla(assembly, extra_metadata)
                        }
                    };

                    let contract = Contract::new(
                        full_path.clone(),
                        source,
                        contract.metadata.to_owned().expect("Always exists"),
                    );
                    Some((full_path, Ok(contract)))
                },
            )
            .collect::<BTreeMap<String, anyhow::Result<Contract>>>();

        let mut contracts = BTreeMap::new();
        for (path, result) in results.into_iter() {
            match result {
                Ok(contract) => {
                    contracts.insert(path, contract);
                }
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
        libraries: BTreeMap<String, BTreeMap<String, String>>,
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
        libraries: BTreeMap<String, BTreeMap<String, String>>,
        mut solc_output: Option<&mut SolcStandardJsonOutput>,
        solc_version: Option<&SolcVersion>,
        debug_config: Option<&era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<Self> {
        let results = sources
            .into_par_iter()
            .map(|(path, mut source)| {
                let source_code = match source.try_resolve() {
                    Ok(()) => source.take_content().expect("Always exists"),
                    Err(error) => return (path, Err(error)),
                };

                let hash: [u8; era_compiler_common::BYTE_LENGTH_FIELD] =
                    sha3::Keccak256::digest(source_code.as_bytes()).into();

                if let Some(debug_config) = debug_config {
                    if let Err(error) =
                        debug_config.dump_yul(path.as_str(), None, source_code.as_str())
                    {
                        return (path, Err(error));
                    }
                }

                let mut lexer = Lexer::new(source_code);
                let object = match Object::parse(&mut lexer, None)
                    .map_err(|error| anyhow::anyhow!("Yul parsing: {error:?}"))
                {
                    Ok(object) => object,
                    Err(error) => return (path, Err(error)),
                };

                let contract = Contract::new(
                    path.clone(),
                    ContractIR::new_yul(object),
                    serde_json::json!({
                        "source_hash": hex::encode(hash),
                        "solc_version": solc_version,
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
        solc_output: Option<&mut SolcStandardJsonOutput>,
    ) -> anyhow::Result<Self> {
        let sources = paths
            .iter()
            .map(|path| {
                let source = SolcStandardJsonInputSource::from(path.as_path());
                (path.to_string_lossy().to_string(), source)
            })
            .collect::<BTreeMap<String, SolcStandardJsonInputSource>>();
        Self::try_from_llvm_ir_sources(sources, solc_output)
    }

    ///
    /// Parses the LLVM IR `sources` and returns an LLVM IR project.
    ///
    pub fn try_from_llvm_ir_sources(
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

                let hash: [u8; era_compiler_common::BYTE_LENGTH_FIELD] =
                    sha3::Keccak256::digest(source_code.as_bytes()).into();

                let contract = Contract::new(
                    path.clone(),
                    ContractIR::new_llvm_ir(path.clone(), source_code),
                    serde_json::json!({
                        "source_hash": hex::encode(hash),
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
            BTreeMap::new(),
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

                let hash: [u8; era_compiler_common::BYTE_LENGTH_FIELD] =
                    sha3::Keccak256::digest(source_code.as_bytes()).into();

                let contract = Contract::new(
                    path.clone(),
                    ContractIR::new_eravm_assembly(path.clone(), source_code),
                    serde_json::json!({
                        "source_hash": hex::encode(hash),
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
            BTreeMap::new(),
        ))
    }

    ///
    /// Compiles all contracts to EraVM, returning their build artifacts.
    ///
    pub fn compile_to_eravm(
        self,
        messages: &mut Vec<SolcStandardJsonOutputError>,
        enable_eravm_extensions: bool,
        include_metadata_hash: bool,
        bytecode_encoding: zkevm_assembly::RunningVmEncodingMode,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        output_assembly: bool,
        threads: Option<usize>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<EraVMBuild> {
        let identifier_paths = self.identifier_paths.clone();
        let dependency_data = EraVMProcessInputDependencyData::new(
            self.solc_version,
            self.identifier_paths.clone(),
            self.libraries.clone(),
        );

        let input_template = EraVMProcessInput::new(
            None,
            dependency_data,
            enable_eravm_extensions,
            include_metadata_hash,
            bytecode_encoding == zkevm_assembly::RunningVmEncodingMode::Testing,
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
            if let Ok(ref contract) = result {
                hashes.insert(path.to_owned(), contract.build.bytecode_hash.to_owned());
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
                        let hash = hashes
                            .get(dependency_path.as_str())
                            .cloned()
                            .unwrap_or_else(|| {
                                panic!("dependency `{dependency_path}` not found in the project")
                            });
                        contract
                            .build
                            .factory_dependencies
                            .insert(hash, dependency_path);
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
        include_metadata_hash: bool,
        threads: Option<usize>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<EVMBuild> {
        let dependency_data = EVMProcessInputDependencyData::new(
            self.solc_version,
            self.identifier_paths,
            self.libraries,
        );

        let input_template = EVMProcessInput::new(
            None,
            dependency_data,
            include_metadata_hash,
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
