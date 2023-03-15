//!
//! The contract data representation.
//!

pub mod ir;
pub mod metadata;
pub mod state;

use std::collections::HashSet;
use std::sync::Arc;
use std::sync::RwLock;

use compiler_llvm_context::WriteLLVM;
use sha3::Digest;

use crate::build::contract::Contract as ContractBuild;
use crate::project::Project;

use self::ir::IR;
use self::metadata::Metadata;
use self::state::State;

///
/// The contract data representation.
///
#[derive(Debug, Clone)]
pub struct Contract {
    /// The absolute file path.
    pub path: String,
    /// The IR source code data.
    pub ir: IR,
    /// The metadata.
    pub metadata: serde_json::Value,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        path: String,
        source_hash: [u8; compiler_common::BYTE_LENGTH_FIELD],
        source_version: semver::Version,
        ir: IR,
        metadata: Option<serde_json::Value>,
    ) -> Self {
        Self {
            path,
            ir,
            metadata: metadata.unwrap_or_else(|| {
                serde_json::json!({
                    "source_hash": hex::encode(source_hash.as_slice()),
                    "source_version": source_version.to_string(),
                })
            }),
        }
    }

    ///
    /// Returns the contract identifier, which is:
    /// - the Yul object identifier for Yul
    /// - the full contract path for EVM legacy assembly
    /// - the module name for LLVM IR
    ///
    pub fn identifier(&self) -> &str {
        match self.ir {
            IR::Yul(ref yul) => yul.object.identifier.as_str(),
            IR::EVMLA(ref evm) => evm.assembly.full_path(),
            IR::LLVMIR(ref llvm_ir) => llvm_ir.path.as_str(),
        }
    }

    ///
    /// Extract factory dependencies.
    ///
    pub fn drain_factory_dependencies(&mut self) -> HashSet<String> {
        match self.ir {
            IR::Yul(ref mut yul) => yul.object.factory_dependencies.drain().collect(),
            IR::EVMLA(ref mut evm) => evm.assembly.factory_dependencies.drain().collect(),
            IR::LLVMIR(_) => HashSet::new(),
        }
    }

    ///
    /// Compiles the specified contract, setting its build artifacts.
    ///
    pub fn compile(
        mut self,
        project: Arc<RwLock<Project>>,
        target_machine: compiler_llvm_context::TargetMachine,
        optimizer_settings: compiler_llvm_context::OptimizerSettings,
        is_system_mode: bool,
        debug_config: Option<compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<ContractBuild> {
        let llvm = inkwell::context::Context::create();
        let optimizer = compiler_llvm_context::Optimizer::new(target_machine, optimizer_settings);

        let metadata = Metadata::new(
            self.metadata.take(),
            semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid"),
            optimizer.settings().to_owned(),
        );
        let metadata_json = serde_json::to_value(&metadata).expect("Always valid");
        let metadata_string = serde_json::to_string(&metadata).expect("Always valid");
        let metadata_hash: [u8; compiler_common::BYTE_LENGTH_FIELD] =
            sha3::Keccak256::digest(metadata_string.as_bytes()).into();

        let module = match self.ir {
            IR::LLVMIR(ref llvm_ir) => {
                let memory_buffer =
                    inkwell::memory_buffer::MemoryBuffer::create_from_memory_range_copy(
                        llvm_ir.source.as_bytes(),
                        self.path.as_str(),
                    );
                let module = llvm
                    .create_module_from_ir(memory_buffer)
                    .map_err(|error| anyhow::anyhow!(error.to_string()))?;
                module
            }
            _ => llvm.create_module(self.path.as_str()),
        };
        let mut context = compiler_llvm_context::Context::new(
            &llvm,
            module,
            optimizer,
            Some(project.clone()),
            debug_config,
        );
        context.set_solidity_data(compiler_llvm_context::ContextSolidityData::default());
        match self.ir {
            IR::Yul(_) => {
                let yul_data = compiler_llvm_context::ContextYulData::new(is_system_mode);
                context.set_yul_data(yul_data);
            }
            IR::EVMLA(_) => {
                let version = project.read().expect("Sync").version.to_owned();
                let evmla_data = compiler_llvm_context::ContextEVMLAData::new(version);
                context.set_evmla_data(evmla_data);
            }
            IR::LLVMIR(_) => {}
        }

        let identifier = self.identifier().to_owned();
        let factory_dependencies = self.drain_factory_dependencies();

        self.ir.declare(&mut context).map_err(|error| {
            anyhow::anyhow!(
                "The contract `{}` LLVM IR generator declaration pass error: {}",
                self.path,
                error
            )
        })?;
        self.ir.into_llvm(&mut context).map_err(|error| {
            anyhow::anyhow!(
                "The contract `{}` LLVM IR generator definition pass error: {}",
                self.path,
                error
            )
        })?;

        let mut build = context.build(self.path.as_str(), metadata_hash)?;

        for dependency in factory_dependencies.into_iter() {
            let full_path = project
                .read()
                .expect("Sync")
                .identifier_paths
                .get(dependency.as_str())
                .cloned()
                .unwrap_or_else(|| panic!("Dependency `{dependency}` full path not found"));
            let hash = match project
                .read()
                .expect("Sync")
                .contract_states
                .get(full_path.as_str())
            {
                Some(State::Build(build)) => build.build.bytecode_hash.to_owned(),
                Some(_) => {
                    panic!("Dependency `{full_path}` must be built at this point")
                }
                None => anyhow::bail!(
                    "Dependency contract `{}` not found in the project",
                    full_path
                ),
            };
            build.factory_dependencies.insert(hash, full_path);
        }

        Ok(ContractBuild::new(
            self.path,
            identifier,
            build,
            metadata_json,
        ))
    }
}

impl<D> WriteLLVM<D> for Contract
where
    D: compiler_llvm_context::Dependency,
{
    fn declare(&mut self, context: &mut compiler_llvm_context::Context<D>) -> anyhow::Result<()> {
        self.ir.declare(context)
    }

    fn into_llvm(self, context: &mut compiler_llvm_context::Context<D>) -> anyhow::Result<()> {
        self.ir.into_llvm(context)
    }
}
