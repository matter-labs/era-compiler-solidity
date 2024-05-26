//!
//! The processed input data.
//!

pub mod contract;

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;

use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use serde::Deserialize;
use serde::Serialize;
use sha3::Digest;

use crate::build_eravm::contract::Contract as EraVMContractBuild;
use crate::build_eravm::Build as EraVMBuild;
use crate::build_evm::contract::Contract as EVMContractBuild;
use crate::build_evm::Build as EVMBuild;
use crate::missing_libraries::MissingLibraries;
use crate::process::input_eravm::Input as EraVMProcessInput;
use crate::process::input_evm::Input as EVMProcessInput;
use crate::process::output_eravm::Output as EraVMProcessOutput;
use crate::process::output_evm::Output as EVMProcessOutput;
use crate::project::contract::ir::IR as ProjectContractIR;
use crate::project::contract::ir::IR;
use crate::project::contract::Contract as ProjectContract;
use crate::solc::pipeline::Pipeline as SolcPipeline;
use crate::solc::standard_json::input::language::Language as SolcStandardJsonInputLanguage;
use crate::solc::standard_json::output::Output as SolcStandardJsonOutput;
use crate::solc::version::Version as SolcVersion;
use crate::solc::Compiler as SolcCompiler;
use crate::yul::lexer::Lexer;
use crate::yul::parser::statement::object::Object;

use self::contract::Contract;

///
/// The processes input data.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    /// The project language.
    pub language: SolcStandardJsonInputLanguage,
    /// The `solc` compiler version.
    pub solc_version: Option<SolcVersion>,
    /// The project contracts,
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
    pub fn try_from_solidity_sources(
        solc_output: &mut SolcStandardJsonOutput,
        sources: BTreeMap<String, String>,
        libraries: BTreeMap<String, BTreeMap<String, String>>,
        pipeline: SolcPipeline,
        solc_compiler: &SolcCompiler,
        debug_config: Option<&era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<Self> {
        if let SolcPipeline::EVMLA = pipeline {
            solc_output.preprocess_dependencies()?;
        }

        let files = match solc_output.contracts.as_ref() {
            Some(files) => files,
            None => {
                anyhow::bail!(
                    "{}",
                    solc_output
                        .errors
                        .as_ref()
                        .map(|errors| serde_json::to_string_pretty(errors).expect("Always valid"))
                        .unwrap_or_else(|| "Unknown project assembling error".to_owned())
                );
            }
        };
        let mut project_contracts = BTreeMap::new();

        let solc_version = solc_compiler.version.to_owned();

        for (path, contracts) in files.iter() {
            for (name, contract) in contracts.iter() {
                let full_path = format!("{path}:{name}");

                let source = match pipeline {
                    SolcPipeline::Yul => {
                        let ir_optimized = match contract.ir_optimized.to_owned() {
                            Some(ir_optimized) => ir_optimized,
                            None => continue,
                        };
                        if ir_optimized.is_empty() {
                            continue;
                        }

                        if let Some(debug_config) = debug_config {
                            debug_config.dump_yul(
                                full_path.as_str(),
                                None,
                                ir_optimized.as_str(),
                            )?;
                        }

                        let mut lexer = Lexer::new(ir_optimized.to_owned());
                        let object = Object::parse(&mut lexer, None).map_err(|error| {
                            anyhow::anyhow!("Contract `{}` parsing error: {:?}", full_path, error)
                        })?;

                        ProjectContractIR::new_yul(ir_optimized.to_owned(), object)
                    }
                    SolcPipeline::EVMLA => {
                        let evm = contract.evm.as_ref();
                        let assembly = match evm.and_then(|evm| evm.assembly.to_owned()) {
                            Some(assembly) => assembly.to_owned(),
                            None => continue,
                        };
                        let extra_metadata = evm
                            .and_then(|evm| evm.extra_metadata.to_owned())
                            .unwrap_or_default();

                        ProjectContractIR::new_evmla(assembly, extra_metadata)
                    }
                };

                let source_code = sources
                    .get(path.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Source code for path `{}` not found", path))?;
                let hash = sha3::Keccak256::digest(source_code.as_bytes()).into();

                let project_contract = ProjectContract::new(
                    full_path.clone(),
                    source,
                    contract.metadata.to_owned(),
                    hash,
                    Some(&solc_version),
                );
                project_contracts.insert(full_path, project_contract);
            }
        }

        Ok(Project::new(
            SolcStandardJsonInputLanguage::Solidity,
            Some(solc_version.to_owned()),
            project_contracts,
            libraries,
        ))
    }

    ///
    /// Reads the Yul source code `paths` and returns a Yul project.
    ///
    pub fn try_from_yul_paths(
        paths: &[PathBuf],
        libraries: BTreeMap<String, BTreeMap<String, String>>,
        solc_version: Option<&SolcVersion>,
        debug_config: Option<&era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<Self> {
        let sources = paths
            .iter()
            .map(|path| {
                let source_code = std::fs::read_to_string(path.as_path())
                    .map_err(|error| anyhow::anyhow!("Yul file {path:?} reading error: {error}"))?;
                Ok((path.to_string_lossy().to_string(), source_code))
            })
            .collect::<anyhow::Result<BTreeMap<String, String>>>()?;
        Self::try_from_yul_sources(sources, libraries, solc_version, debug_config)
    }

    ///
    /// Parses the Yul `sources` and returns a Yul project.
    ///
    pub fn try_from_yul_sources(
        sources: BTreeMap<String, String>,
        libraries: BTreeMap<String, BTreeMap<String, String>>,
        solc_version: Option<&SolcVersion>,
        debug_config: Option<&era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<Self> {
        let project_contracts = sources
            .into_par_iter()
            .map(|(path, source_code)| {
                let hash = sha3::Keccak256::digest(source_code.as_bytes()).into();

                let mut lexer = Lexer::new(source_code.to_owned());
                let object = Object::parse(&mut lexer, None).map_err(|error| {
                    anyhow::anyhow!("Yul object `{}` parsing error: {}", path, error)
                })?;

                if let Some(debug_config) = debug_config {
                    debug_config.dump_yul(path.as_str(), None, source_code.as_str())?;
                }

                let contract = Contract::new(
                    path.clone(),
                    IR::new_yul(source_code.to_owned(), object),
                    None,
                    hash,
                    solc_version,
                );

                Ok((path, contract))
            })
            .collect::<anyhow::Result<BTreeMap<String, Contract>>>()?;

        Ok(Self::new(
            SolcStandardJsonInputLanguage::Yul,
            solc_version.cloned(),
            project_contracts,
            libraries,
        ))
    }

    ///
    /// Reads the LLVM IR source code `paths` and returns an LLVM IR project.
    ///
    pub fn try_from_llvm_ir_paths(paths: &[PathBuf]) -> anyhow::Result<Self> {
        let sources = paths
            .iter()
            .map(|path| {
                let source_code = std::fs::read_to_string(path.as_path()).map_err(|error| {
                    anyhow::anyhow!("LLVM IR file {path:?} reading error: {error}")
                })?;
                Ok((path.to_string_lossy().to_string(), source_code))
            })
            .collect::<anyhow::Result<BTreeMap<String, String>>>()?;
        Self::try_from_llvm_ir_sources(sources)
    }

    ///
    /// Parses the LLVM IR `sources` and returns an LLVM IR project.
    ///
    pub fn try_from_llvm_ir_sources(sources: BTreeMap<String, String>) -> anyhow::Result<Self> {
        let project_contracts = sources
            .into_par_iter()
            .map(|(path, source_code)| {
                let hash = sha3::Keccak256::digest(source_code.as_bytes()).into();

                let contract = Contract::new(
                    path.clone(),
                    IR::new_llvm_ir(path.clone(), source_code),
                    None,
                    hash,
                    None,
                );

                Ok((path, contract))
            })
            .collect::<anyhow::Result<BTreeMap<String, Contract>>>()?;

        Ok(Self::new(
            SolcStandardJsonInputLanguage::LLVMIR,
            None,
            project_contracts,
            BTreeMap::new(),
        ))
    }

    ///
    /// Reads the EraVM assembly source code `paths` and returns an EraVM assembly project.
    ///
    pub fn try_from_eravm_assembly_paths(paths: &[PathBuf]) -> anyhow::Result<Self> {
        let sources = paths
            .iter()
            .map(|path| {
                let source_code = std::fs::read_to_string(path.as_path()).map_err(|error| {
                    anyhow::anyhow!("EraVM assembly file {path:?} reading error: {error}")
                })?;
                Ok((path.to_string_lossy().to_string(), source_code))
            })
            .collect::<anyhow::Result<BTreeMap<String, String>>>()?;
        Self::try_from_eravm_assembly_sources(sources)
    }

    ///
    /// Parses the EraVM assembly `sources` and returns an EraVM assembly project.
    ///
    pub fn try_from_eravm_assembly_sources(
        sources: BTreeMap<String, String>,
    ) -> anyhow::Result<Self> {
        let project_contracts = sources
            .into_par_iter()
            .map(|(path, source_code)| {
                let hash = sha3::Keccak256::digest(source_code.as_bytes()).into();

                let contract = Contract::new(
                    path.clone(),
                    IR::new_eravm_assembly(path.clone(), source_code),
                    None,
                    hash,
                    None,
                );

                Ok((path, contract))
            })
            .collect::<anyhow::Result<BTreeMap<String, Contract>>>()?;

        Ok(Self::new(
            SolcStandardJsonInputLanguage::EraVMAssembly,
            None,
            project_contracts,
            BTreeMap::new(),
        ))
    }

    ///
    /// Compiles all contracts to EraVM, returning their build artifacts.
    ///
    pub fn compile_to_eravm(
        self,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        is_system_mode: bool,
        include_metadata_hash: bool,
        bytecode_encoding: zkevm_assembly::RunningVmEncodingMode,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<EraVMBuild> {
        let results: BTreeMap<String, anyhow::Result<EraVMContractBuild>> = self
            .contracts
            .par_iter()
            .map(|(full_path, contract)| {
                let process_output: anyhow::Result<EraVMProcessOutput> = crate::process::call(
                    EraVMProcessInput::new(
                        Cow::Borrowed(contract),
                        Cow::Borrowed(&self),
                        is_system_mode,
                        include_metadata_hash,
                        bytecode_encoding == zkevm_assembly::RunningVmEncodingMode::Testing,
                        optimizer_settings.clone(),
                        debug_config.clone(),
                    ),
                    era_compiler_llvm_context::Target::EraVM,
                );

                (
                    full_path.to_owned(),
                    process_output.map(|output| output.build),
                )
            })
            .collect();

        let mut build = EraVMBuild::default();
        let mut hashes = HashMap::with_capacity(results.len());
        for (path, result) in results.iter() {
            match result {
                Ok(contract) => {
                    hashes.insert(path.to_owned(), contract.build.bytecode_hash.to_owned());
                }
                Err(error) => {
                    anyhow::bail!("Contract `{}` compiling error: {:?}", path, error);
                }
            }
        }
        for (path, result) in results.into_iter() {
            match result {
                Ok(mut contract) => {
                    for dependency in contract.factory_dependencies.drain() {
                        let dependency_path = self
                            .identifier_paths
                            .get(dependency.as_str())
                            .cloned()
                            .unwrap_or_else(|| {
                                panic!("Dependency `{dependency}` full path not found")
                            });
                        let hash = match hashes.get(dependency_path.as_str()) {
                            Some(hash) => hash.to_owned(),
                            None => anyhow::bail!(
                                "Dependency contract `{}` not found in the project",
                                dependency_path
                            ),
                        };
                        contract
                            .build
                            .factory_dependencies
                            .insert(hash, dependency_path);
                    }

                    build.contracts.insert(path, contract);
                }
                Err(error) => {
                    anyhow::bail!("Contract `{}` compiling error: {:?}", path, error);
                }
            }
        }

        Ok(build)
    }

    ///
    /// Compiles all contracts to EVM, returning their build artifacts.
    ///
    pub fn compile_to_evm(
        self,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        include_metadata_hash: bool,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<EVMBuild> {
        let results: BTreeMap<String, anyhow::Result<EVMContractBuild>> = self
            .contracts
            .par_iter()
            .map(|(full_path, contract)| {
                let process_output: anyhow::Result<EVMProcessOutput> = crate::process::call(
                    EVMProcessInput::new(
                        Cow::Borrowed(contract),
                        Cow::Borrowed(&self),
                        include_metadata_hash,
                        optimizer_settings.clone(),
                        debug_config.clone(),
                    ),
                    era_compiler_llvm_context::Target::EVM,
                );

                (
                    full_path.to_owned(),
                    process_output.map(|output| output.build),
                )
            })
            .collect();

        let mut build = EVMBuild::default();
        for (path, result) in results.into_iter() {
            match result {
                Ok(contract) => {
                    build.contracts.insert(path, contract);
                }
                Err(error) => {
                    anyhow::bail!("Contract `{}` compiling error: {:?}", path, error);
                }
            }
        }

        Ok(build)
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

impl era_compiler_llvm_context::EraVMDependency for Project {
    fn compile(
        project: Self,
        identifier: &str,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        is_system_mode: bool,
        include_metadata_hash: bool,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<String> {
        let contract_path = project.resolve_path(identifier)?;
        let contract = project
            .contracts
            .get(contract_path.as_str())
            .cloned()
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Dependency contract `{}` not found in the project",
                    contract_path
                )
            })?;

        contract
            .compile_to_eravm(
                project,
                optimizer_settings,
                is_system_mode,
                include_metadata_hash,
                debug_config,
            )
            .map_err(|error| {
                anyhow::anyhow!(
                    "Dependency contract `{}` compiling error: {}",
                    identifier,
                    error
                )
            })
            .map(|contract| contract.build.bytecode_hash)
    }

    fn resolve_path(&self, identifier: &str) -> anyhow::Result<String> {
        self.identifier_paths
            .get(identifier.strip_suffix("_deployed").unwrap_or(identifier))
            .cloned()
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Contract with identifier `{}` not found in the project",
                    identifier
                )
            })
    }

    fn resolve_library(&self, path: &str) -> anyhow::Result<String> {
        for (file_path, contracts) in self.libraries.iter() {
            for (contract_name, address) in contracts.iter() {
                let key = format!("{file_path}:{contract_name}");
                if key.as_str() == path {
                    return Ok(address["0x".len()..].to_owned());
                }
            }
        }

        anyhow::bail!("Library `{}` not found in the project", path);
    }
}

impl era_compiler_llvm_context::EVMDependency for Project {
    fn compile(
        _project: Self,
        _identifier: &str,
        _optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        _include_metadata_hash: bool,
        _debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<String> {
        todo!()
    }

    fn resolve_path(&self, _identifier: &str) -> anyhow::Result<String> {
        todo!()
    }

    fn resolve_library(&self, _path: &str) -> anyhow::Result<String> {
        todo!()
    }
}
