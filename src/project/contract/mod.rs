//!
//! The contract data representation.
//!

pub mod source;
pub mod state;

use std::collections::BTreeMap;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::RwLock;

use compiler_llvm_context::WriteLLVM;

use crate::build::contract::Contract as ContractBuild;
use crate::project::Project;
use crate::solc::standard_json::output::contract::Contract as SolcStandardJsonOutputContract;

use self::source::Source;
use self::state::State;

///
/// The contract data representation.
///
#[derive(Debug, Clone)]
pub struct Contract {
    /// The absolute file path.
    pub path: String,
    /// The source code data.
    pub source: Source,
    /// The ABI specification.
    pub abi: Option<serde_json::Value>,
    /// The method identifiers.
    pub method_identifiers: Option<BTreeMap<String, String>>,
    /// The storage layout.
    pub storage_layout: Option<serde_json::Value>,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        path: String,
        source: Source,
        mut contract: Option<&mut SolcStandardJsonOutputContract>,
    ) -> Self {
        Self {
            path,
            source,
            abi: contract.as_mut().and_then(|contract| contract.abi.take()),
            method_identifiers: contract
                .as_mut()
                .and_then(|contract| contract.evm.as_mut())
                .and_then(|evm| evm.method_identifiers.take()),
            storage_layout: contract
                .as_mut()
                .and_then(|contract| contract.storage_layout.take()),
        }
    }

    ///
    /// Returns the contract identifier, which is:
    /// - the Yul object identifier for Yul
    /// - the full contract path for EVM legacy assembly
    /// - the module name for LLVM IR
    ///
    pub fn identifier(&self) -> &str {
        match self.source {
            Source::Yul(ref yul) => yul.object.identifier.as_str(),
            Source::EVMLA(ref evm) => evm.assembly.full_path(),
            Source::LLVMIR(ref llvm_ir) => llvm_ir.path.as_str(),
        }
    }

    ///
    /// Extract factory dependencies.
    ///
    pub fn drain_factory_dependencies(&mut self) -> HashSet<String> {
        match self.source {
            Source::Yul(ref mut yul) => yul.object.factory_dependencies.drain().collect(),
            Source::EVMLA(ref mut evm) => evm.assembly.factory_dependencies.drain().collect(),
            Source::LLVMIR(_) => HashSet::new(),
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
        let module = match self.source {
            Source::LLVMIR(ref llvm_ir) => {
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
        let optimizer = compiler_llvm_context::Optimizer::new(target_machine, optimizer_settings);
        let mut context = compiler_llvm_context::Context::new(
            &llvm,
            module,
            optimizer,
            Some(project.clone()),
            debug_config,
        );
        context.set_solidity_data(compiler_llvm_context::ContextSolidityData::default());
        match self.source {
            Source::Yul(_) => {
                let yul_data = compiler_llvm_context::ContextYulData::new(is_system_mode);
                context.set_yul_data(yul_data);
            }
            Source::EVMLA(_) => {
                let version = project.read().expect("Sync").version.to_owned();
                let evmla_data = compiler_llvm_context::ContextEVMLAData::new(version);
                context.set_evmla_data(evmla_data);
            }
            Source::LLVMIR(_) => {}
        }

        let identifier = self.identifier().to_owned();
        let factory_dependencies = self.drain_factory_dependencies();

        self.source.declare(&mut context).map_err(|error| {
            anyhow::anyhow!(
                "The contract `{}` LLVM IR generator declaration pass error: {}",
                self.path,
                error
            )
        })?;
        self.source.into_llvm(&mut context).map_err(|error| {
            anyhow::anyhow!(
                "The contract `{}` LLVM IR generator definition pass error: {}",
                self.path,
                error
            )
        })?;

        let mut build = context.build(self.path.as_str())?;
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
                Some(State::Build(build)) => build.build.hash.to_owned(),
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
            self.abi,
            self.method_identifiers,
            self.storage_layout,
        ))
    }
}

impl<D> WriteLLVM<D> for Contract
where
    D: compiler_llvm_context::Dependency,
{
    fn declare(&mut self, context: &mut compiler_llvm_context::Context<D>) -> anyhow::Result<()> {
        self.source.declare(context)
    }

    fn into_llvm(self, context: &mut compiler_llvm_context::Context<D>) -> anyhow::Result<()> {
        self.source.into_llvm(context)
    }
}
