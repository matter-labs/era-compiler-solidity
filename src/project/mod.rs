//!
//! The processed input data representation.
//!

pub mod contract;

use std::collections::BTreeMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;

use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use sha3::Digest;

use crate::build::Build;
use crate::project::contract::ir::IR;
use crate::project::contract::state::State;
use crate::solc::Compiler as SolcCompiler;
use crate::yul::lexer::Lexer;
use crate::yul::parser::statement::object::Object;

use self::contract::state::State as ContractState;
use self::contract::Contract;

///
/// The processes input data representation.
///
#[derive(Debug)]
pub struct Project {
    /// The source code version.
    pub version: semver::Version,
    /// The contract data,
    pub contract_states: BTreeMap<String, ContractState>,
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
        version: semver::Version,
        contracts: BTreeMap<String, Contract>,
        libraries: BTreeMap<String, BTreeMap<String, String>>,
    ) -> Self {
        let mut identifier_paths = BTreeMap::new();
        for (path, contract) in contracts.iter() {
            identifier_paths.insert(contract.identifier().to_owned(), path.to_owned());
        }

        Self {
            version,
            contract_states: contracts
                .into_iter()
                .map(|(path, contract)| (path, ContractState::Source(contract)))
                .collect(),
            identifier_paths,
            libraries,
        }
    }

    ///
    /// Compiles the specified contract, setting its build artifacts.
    ///
    pub fn compile(
        project: Arc<RwLock<Self>>,
        contract_path: &str,
        target_machine: compiler_llvm_context::TargetMachine,
        optimizer_settings: compiler_llvm_context::OptimizerSettings,
        is_system_mode: bool,
        debug_config: Option<compiler_llvm_context::DebugConfig>,
    ) {
        let mut project_guard = project.write().expect("Sync");
        match project_guard
            .contract_states
            .remove(contract_path)
            .expect("Always exists")
        {
            ContractState::Source(contract) => {
                let waiter = ContractState::waiter();
                project_guard.contract_states.insert(
                    contract_path.to_owned(),
                    ContractState::Waiter(waiter.clone()),
                );
                std::mem::drop(project_guard);

                match contract.compile(
                    project.clone(),
                    target_machine,
                    optimizer_settings,
                    is_system_mode,
                    debug_config,
                ) {
                    Ok(build) => {
                        project
                            .write()
                            .expect("Sync")
                            .contract_states
                            .insert(contract_path.to_owned(), ContractState::Build(build));
                        waiter.1.notify_all();
                    }
                    Err(error) => {
                        project
                            .write()
                            .expect("Sync")
                            .contract_states
                            .insert(contract_path.to_owned(), ContractState::Error(error));
                        waiter.1.notify_all();
                    }
                }
            }
            ContractState::Waiter(waiter) => {
                project_guard.contract_states.insert(
                    contract_path.to_owned(),
                    ContractState::Waiter(waiter.clone()),
                );
                std::mem::drop(project_guard);

                let _guard = waiter.1.wait(waiter.0.lock().expect("Sync"));
            }
            ContractState::Build(build) => {
                project_guard
                    .contract_states
                    .insert(contract_path.to_owned(), ContractState::Build(build));
            }
            ContractState::Error(error) => {
                project_guard
                    .contract_states
                    .insert(contract_path.to_owned(), ContractState::Error(error));
            }
        }
    }

    ///
    /// Compiles all contracts, returning their build artifacts.
    ///
    #[allow(clippy::needless_collect)]
    pub fn compile_all(
        self,
        target_machine: compiler_llvm_context::TargetMachine,
        optimizer_settings: compiler_llvm_context::OptimizerSettings,
        is_system_mode: bool,
        debug_config: Option<compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<Build> {
        let project = Arc::new(RwLock::new(self));

        let contract_paths: Vec<String> = project
            .read()
            .expect("Sync")
            .contract_states
            .keys()
            .cloned()
            .collect();
        let _: Vec<()> = contract_paths
            .into_par_iter()
            .map(|contract_path| {
                Self::compile(
                    project.clone(),
                    contract_path.as_str(),
                    target_machine.clone(),
                    optimizer_settings.clone(),
                    is_system_mode,
                    debug_config.clone(),
                );
            })
            .collect();

        let project = Arc::try_unwrap(project)
            .expect("No other references must exist at this point")
            .into_inner()
            .expect("Sync");
        let mut build = Build::default();
        for (path, state) in project.contract_states.into_iter() {
            match state {
                State::Build(contract_build) => {
                    build.contracts.insert(path, contract_build);
                }
                State::Error(error) => return Err(error),
                _ => panic!("Contract `{path}` must be built at this point"),
            }
        }
        Ok(build)
    }

    ///
    /// Parses the Yul source code file and returns the source data.
    ///
    pub fn try_from_yul_path(path: &Path) -> anyhow::Result<Self> {
        let source_code = std::fs::read_to_string(path)
            .map_err(|error| anyhow::anyhow!("Yul file {:?} reading error: {}", path, error))?;
        let source_hash = sha3::Keccak256::digest(source_code.as_bytes()).into();

        let mut lexer = Lexer::new(source_code.clone());
        let path = path.to_string_lossy().to_string();
        let object = Object::parse(&mut lexer, None)
            .map_err(|error| anyhow::anyhow!("Yul object `{}` parsing error: {}", path, error,))?;

        let mut project_contracts = BTreeMap::new();
        project_contracts.insert(
            path.clone(),
            Contract::new(
                path,
                source_hash,
                SolcCompiler::LAST_SUPPORTED_VERSION,
                IR::new_yul(source_code, object),
                None,
            ),
        );

        Ok(Self::new(
            SolcCompiler::LAST_SUPPORTED_VERSION,
            project_contracts,
            BTreeMap::new(),
        ))
    }

    ///
    /// Parses the test Yul source code string and returns the source data.
    ///
    /// Only for integration testing purposes.
    ///
    pub fn try_from_yul_string(path: &str, source_code: &str) -> anyhow::Result<Self> {
        let source_hash = sha3::Keccak256::digest(source_code.as_bytes()).into();

        let mut lexer = Lexer::new(source_code.to_owned());
        let object = Object::parse(&mut lexer, None)
            .map_err(|error| anyhow::anyhow!("Yul object `{}` parsing error: {}", path, error))?;

        let mut project_contracts = BTreeMap::new();
        project_contracts.insert(
            path.to_owned(),
            Contract::new(
                path.to_owned(),
                source_hash,
                SolcCompiler::LAST_SUPPORTED_VERSION,
                IR::new_yul(source_code.to_owned(), object),
                None,
            ),
        );

        Ok(Self::new(
            SolcCompiler::LAST_SUPPORTED_VERSION,
            project_contracts,
            BTreeMap::new(),
        ))
    }

    ///
    /// Parses the LLVM IR source code file and returns the source data.
    ///
    pub fn try_from_llvm_ir_path(path: &Path) -> anyhow::Result<Self> {
        let source_code = std::fs::read_to_string(path)
            .map_err(|error| anyhow::anyhow!("LLVM IR file {:?} reading error: {}", path, error))?;
        let source_hash = sha3::Keccak256::digest(source_code.as_bytes()).into();

        let path = path.to_string_lossy().to_string();

        let mut project_contracts = BTreeMap::new();
        project_contracts.insert(
            path.clone(),
            Contract::new(
                path.clone(),
                source_hash,
                compiler_llvm_context::LLVM_VERSION,
                IR::new_llvm_ir(path, source_code),
                None,
            ),
        );

        Ok(Self::new(
            compiler_llvm_context::LLVM_VERSION,
            project_contracts,
            BTreeMap::new(),
        ))
    }
}

impl Clone for Project {
    fn clone(&self) -> Self {
        let states = self
            .contract_states
            .iter()
            .map(|(path, state)| {
                let state = match state {
                    ContractState::Source(contract) => ContractState::Source(contract.clone()),
                    _ => {
                        panic!("The project cannot be cloned when the building has already started")
                    }
                };
                (path.clone(), state)
            })
            .collect();

        Self {
            version: self.version.clone(),
            contract_states: states,
            identifier_paths: self.identifier_paths.clone(),
            libraries: self.libraries.clone(),
        }
    }
}

impl compiler_llvm_context::Dependency for Project {
    fn compile(
        project: Arc<RwLock<Self>>,
        identifier: &str,
        target_machine: compiler_llvm_context::TargetMachine,
        optimizer_settings: compiler_llvm_context::OptimizerSettings,
        is_system_mode: bool,
        debug_config: Option<compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<String> {
        let contract_path = project.read().expect("Lock").resolve_path(identifier)?;

        Self::compile(
            project.clone(),
            contract_path.as_str(),
            target_machine,
            optimizer_settings,
            is_system_mode,
            debug_config,
        );

        match project
            .read()
            .expect("Lock")
            .contract_states
            .get(contract_path.as_str())
        {
            Some(ContractState::Build(build)) => Ok(build.build.bytecode_hash.to_owned()),
            Some(ContractState::Error(error)) => anyhow::bail!(
                "Dependency contract `{}` compiling error: {}",
                identifier,
                error
            ),
            Some(_) => panic!("Dependency contract `{contract_path}` must be built at this point"),
            None => anyhow::bail!(
                "Dependency contract `{}` not found in the project",
                contract_path
            ),
        }
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
