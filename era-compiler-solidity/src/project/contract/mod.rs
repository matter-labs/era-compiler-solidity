//!
//! The contract data.
//!

pub mod factory_dependency;
pub mod ir;
pub mod metadata;

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashSet;

use era_compiler_llvm_context::IContext;

use crate::build_eravm::contract::Contract as EraVMContractBuild;
use crate::build_evm::contract::Contract as EVMContractBuild;
use crate::process::input_evm::dependency_data::DependencyData as EVMProcessInputDependencyData;
use crate::yul::parser::wrapper::Wrap;

use self::factory_dependency::FactoryDependency;
use self::ir::IR;
use self::metadata::Metadata;

///
/// The contract data.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Contract {
    /// The contract name.
    pub name: era_compiler_common::ContractName,
    /// The IR source code data.
    pub ir: IR,
    /// The metadata JSON.
    pub source_metadata: serde_json::Value,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        name: era_compiler_common::ContractName,
        ir: IR,
        source_metadata: serde_json::Value,
    ) -> Self {
        Self {
            name,
            ir,
            source_metadata,
        }
    }

    ///
    /// Returns the contract identifier, which is:
    /// - the Yul object identifier for Yul
    /// - the full contract path for EVM legacy assembly
    /// - the module name for LLVM IR
    /// - the full file path for EraVM assembly
    ///
    pub fn identifier(&self) -> &str {
        match self.ir {
            IR::Yul(ref yul) => yul.object.0.identifier.as_str(),
            IR::EVMLA(ref evm) => evm.assembly.full_path(),
            IR::LLVMIR(ref llvm_ir) => llvm_ir.path.as_str(),
            IR::EraVMAssembly(ref eravm_assembly) => eravm_assembly.path.as_str(),
        }
    }

    ///
    /// Compiles the specified contract to EraVM, returning its build artifacts.
    ///
    pub fn compile_to_eravm(
        self,
        solc_version: Option<era_solc::Version>,
        identifier_paths: BTreeMap<String, String>,
        missing_libraries: BTreeSet<String>,
        factory_dependencies: BTreeSet<String>,
        enable_eravm_extensions: bool,
        metadata_hash_type: era_compiler_common::HashType,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        output_assembly: bool,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<EraVMContractBuild> {
        use era_compiler_llvm_context::EraVMWriteLLVM;

        let llvm = inkwell::context::Context::create();
        let optimizer = era_compiler_llvm_context::Optimizer::new(optimizer_settings);

        let metadata = Metadata::new(
            self.source_metadata,
            solc_version
                .as_ref()
                .map(|version| version.default.to_owned()),
            solc_version
                .as_ref()
                .map(|version| version.l2_revision.to_owned()),
            optimizer.settings().to_owned(),
            llvm_options.as_slice(),
        );
        let metadata_json = serde_json::to_value(&metadata).expect("Always valid");
        let metadata_bytes = serde_json::to_vec(&metadata_json).expect("Always valid");
        let metadata_hash = match metadata_hash_type {
            era_compiler_common::HashType::None => None,
            era_compiler_common::HashType::Keccak256 => Some(era_compiler_common::Hash::keccak256(
                metadata_bytes.as_slice(),
            )),
            era_compiler_common::HashType::Ipfs => {
                Some(era_compiler_common::Hash::ipfs(metadata_bytes.as_slice()))
            }
        };

        let build = match self.ir {
            IR::Yul(mut deploy_code) => {
                let module = llvm.create_module(self.name.full_path.as_str());
                let mut context: era_compiler_llvm_context::EraVMContext<
                    '_,
                    era_compiler_llvm_context::DummyDependency,
                > = era_compiler_llvm_context::EraVMContext::new(
                    &llvm,
                    module,
                    llvm_options,
                    optimizer,
                    debug_config,
                );
                context.set_solidity_data(
                    era_compiler_llvm_context::EraVMContextSolidityData::default(),
                );
                let yul_data = era_compiler_llvm_context::EraVMContextYulData::new(
                    enable_eravm_extensions,
                    identifier_paths,
                );
                context.set_yul_data(yul_data);

                deploy_code.declare(&mut context)?;
                deploy_code
                    .into_llvm(&mut context)
                    .map_err(|error| anyhow::anyhow!("LLVM IR generator: {error}"))?;

                context.build(
                    self.name.full_path.as_str(),
                    metadata_hash,
                    output_assembly,
                    false,
                )?
            }
            IR::EVMLA(mut evmla) => {
                let solc_version = solc_version
                    .clone()
                    .expect("The EVM assembly codegen cannot be executed without `solc`");

                let module = llvm.create_module(self.name.full_path.as_str());
                let mut context: era_compiler_llvm_context::EraVMContext<
                    '_,
                    era_compiler_llvm_context::DummyDependency,
                > = era_compiler_llvm_context::EraVMContext::new(
                    &llvm,
                    module,
                    llvm_options,
                    optimizer,
                    debug_config,
                );
                context.set_solidity_data(
                    era_compiler_llvm_context::EraVMContextSolidityData::default(),
                );
                let evmla_data =
                    era_compiler_llvm_context::EraVMContextEVMLAData::new(solc_version.default);
                context.set_evmla_data(evmla_data);

                evmla.declare(&mut context)?;
                evmla
                    .into_llvm(&mut context)
                    .map_err(|error| anyhow::anyhow!("LLVM IR generator: {error}"))?;

                context.build(
                    self.name.full_path.as_str(),
                    metadata_hash,
                    output_assembly,
                    false,
                )?
            }
            IR::LLVMIR(ref llvm_ir) => {
                let memory_buffer =
                    inkwell::memory_buffer::MemoryBuffer::create_from_memory_range_copy(
                        llvm_ir.source.as_bytes(),
                        self.name.full_path.as_str(),
                    );
                let module = llvm
                    .create_module_from_ir(memory_buffer)
                    .map_err(|error| anyhow::anyhow!(error.to_string()))?;
                let context: era_compiler_llvm_context::EraVMContext<
                    '_,
                    era_compiler_llvm_context::DummyDependency,
                > = era_compiler_llvm_context::EraVMContext::new(
                    &llvm,
                    module,
                    llvm_options,
                    optimizer,
                    debug_config,
                );
                context.build(
                    self.name.full_path.as_str(),
                    metadata_hash,
                    output_assembly,
                    false,
                )?
            }
            IR::EraVMAssembly(eravm_assembly) => {
                let target_machine = era_compiler_llvm_context::TargetMachine::new(
                    era_compiler_common::Target::EraVM,
                    optimizer.settings(),
                    llvm_options.as_slice(),
                )?;
                let bytecode_buffer = era_compiler_llvm_context::eravm_assemble(
                    &target_machine,
                    self.name.full_path.as_str(),
                    eravm_assembly.source.as_str(),
                    debug_config.as_ref(),
                )?;
                let assembly_text = if output_assembly {
                    Some(eravm_assembly.source)
                } else {
                    None
                };
                era_compiler_llvm_context::eravm_build(
                    bytecode_buffer,
                    metadata_hash,
                    assembly_text,
                )?
            }
        };

        Ok(EraVMContractBuild::new(
            self.name,
            build,
            metadata_json,
            missing_libraries,
            factory_dependencies,
            era_compiler_common::ObjectFormat::ELF,
        ))
    }

    ///
    /// Compiles the specified contract to EVM, returning its build artifacts.
    ///
    pub fn compile_to_evm(
        self,
        dependency_data: EVMProcessInputDependencyData,
        metadata_hash_type: era_compiler_common::HashType,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<EVMContractBuild> {
        use era_compiler_llvm_context::EVMWriteLLVM;

        let identifier = self.identifier().to_owned();

        let solc_version = dependency_data.solc_version.clone();

        let optimizer = era_compiler_llvm_context::Optimizer::new(optimizer_settings);

        let metadata = Metadata::new(
            self.source_metadata,
            solc_version
                .as_ref()
                .map(|version| version.default.to_owned()),
            solc_version
                .as_ref()
                .map(|version| version.l2_revision.to_owned()),
            optimizer.settings().to_owned(),
            llvm_options.as_slice(),
        );
        let metadata_json = serde_json::to_value(&metadata).expect("Always valid");
        let metadata_bytes = serde_json::to_vec(&metadata_json).expect("Always valid");
        let metadata_hash = match metadata_hash_type {
            era_compiler_common::HashType::None => None,
            era_compiler_common::HashType::Keccak256 => Some(era_compiler_common::Hash::keccak256(
                metadata_bytes.as_slice(),
            )),
            era_compiler_common::HashType::Ipfs => {
                Some(era_compiler_common::Hash::ipfs(metadata_bytes.as_slice()))
            }
        };

        match self.ir {
            IR::Yul(mut deploy_code) => {
                let mut runtime_code = deploy_code
                    .take_runtime_code()
                    .ok_or_else(|| anyhow::anyhow!("Contract `{identifier}` has no runtime code"))?
                    .wrap();

                let deploy_code_identifier = deploy_code.object.0.identifier.clone();
                let runtime_code_identifier = runtime_code.0.identifier.clone();

                let runtime_code_segment = era_compiler_common::CodeSegment::Runtime;
                let runtime_llvm = inkwell::context::Context::create();
                let runtime_module = runtime_llvm.create_module(
                    format!("{}.{runtime_code_segment}", self.name.full_path).as_str(),
                );
                let mut runtime_context = era_compiler_llvm_context::EVMContext::new(
                    &runtime_llvm,
                    runtime_module,
                    llvm_options.clone(),
                    runtime_code_segment,
                    optimizer.clone(),
                    Some(dependency_data.clone()),
                    debug_config.clone(),
                );
                runtime_code.declare(&mut runtime_context)?;
                runtime_code
                    .into_llvm(&mut runtime_context)
                    .map_err(|error| {
                        anyhow::anyhow!("{runtime_code_segment} code LLVM IR generator: {error}",)
                    })?;
                let runtime_buffer = runtime_context.build(self.name.path.as_str())?;

                let deploy_code_segment = era_compiler_common::CodeSegment::Deploy;
                let deploy_llvm = inkwell::context::Context::create();
                let deploy_module = deploy_llvm.create_module(
                    format!("{}.{deploy_code_segment}", self.name.full_path).as_str(),
                );
                let mut deploy_context = era_compiler_llvm_context::EVMContext::new(
                    &deploy_llvm,
                    deploy_module,
                    llvm_options.clone(),
                    deploy_code_segment,
                    optimizer.clone(),
                    Some(dependency_data.clone()),
                    debug_config.clone(),
                );
                deploy_code.declare(&mut deploy_context)?;
                deploy_code
                    .into_llvm(&mut deploy_context)
                    .map_err(|error| {
                        anyhow::anyhow!("{deploy_code_segment} code LLVM IR generator: {error}",)
                    })?;
                let deploy_buffer = deploy_context.build(self.name.path.as_str())?;

                let (deploy_buffer_linked, runtime_buffer_linked) =
                    inkwell::memory_buffer::MemoryBuffer::link_module_evm(
                        &[&deploy_buffer, &runtime_buffer],
                        &[
                            deploy_code_identifier.as_str(),
                            runtime_code_identifier.as_str(),
                        ],
                    )
                    .map_err(|error| anyhow::anyhow!("linking: {error}"))?;

                Ok(EVMContractBuild::new(
                    self.name,
                    identifier,
                    deploy_buffer_linked.as_slice().to_owned(),
                    runtime_buffer_linked.as_slice().to_owned(),
                    metadata_hash,
                    metadata_json,
                ))
            }
            IR::EVMLA(mut deploy_code) => {
                let mut runtime_code_assembly = deploy_code.assembly.get_runtime_code()?.to_owned();
                runtime_code_assembly.set_full_path(deploy_code.assembly.full_path().to_owned());

                let evmla_data = era_compiler_llvm_context::EVMContextEVMLAData::new(
                    solc_version.expect("Always exists").default,
                );

                let runtime_code_segment = era_compiler_common::CodeSegment::Runtime;
                let runtime_llvm = inkwell::context::Context::create();
                let runtime_module = runtime_llvm.create_module(
                    format!("{}.{runtime_code_segment}", self.name.full_path).as_str(),
                );
                let mut runtime_context = era_compiler_llvm_context::EVMContext::new(
                    &runtime_llvm,
                    runtime_module,
                    llvm_options.clone(),
                    runtime_code_segment,
                    optimizer.clone(),
                    Some(dependency_data.clone()),
                    debug_config.clone(),
                );
                runtime_context.set_evmla_data(evmla_data.clone());
                runtime_code_assembly.declare(&mut runtime_context)?;
                runtime_code_assembly
                    .into_llvm(&mut runtime_context)
                    .map_err(|error| {
                        anyhow::anyhow!("{runtime_code_segment} code LLVM IR generator: {error}",)
                    })?;
                let runtime_buffer = runtime_context.build(self.name.path.as_str())?;

                let deploy_code_segment = era_compiler_common::CodeSegment::Deploy;
                let deploy_llvm = inkwell::context::Context::create();
                let deploy_module = deploy_llvm.create_module(
                    format!("{}.{deploy_code_segment}", self.name.full_path).as_str(),
                );
                let mut deploy_context = era_compiler_llvm_context::EVMContext::new(
                    &deploy_llvm,
                    deploy_module,
                    llvm_options.clone(),
                    deploy_code_segment,
                    optimizer.clone(),
                    Some(dependency_data.clone()),
                    debug_config.clone(),
                );
                deploy_context.set_evmla_data(evmla_data);
                deploy_code.declare(&mut deploy_context)?;
                deploy_code
                    .into_llvm(&mut deploy_context)
                    .map_err(|error| {
                        anyhow::anyhow!("{deploy_code_segment} code LLVM IR generator: {error}",)
                    })?;
                let deploy_buffer = deploy_context.build(self.name.path.as_str())?;

                let (deploy_buffer_linked, runtime_buffer_linked) =
                    inkwell::memory_buffer::MemoryBuffer::link_module_evm(
                        &[&deploy_buffer, &runtime_buffer],
                        &[
                            format!("{}.{deploy_code_segment}", self.name.full_path).as_str(),
                            format!("{}.{runtime_code_segment}", self.name.full_path).as_str(),
                        ],
                    )
                    .map_err(|error| anyhow::anyhow!("linking: {error}"))?;

                Ok(EVMContractBuild::new(
                    self.name,
                    identifier,
                    deploy_buffer_linked.as_slice().to_owned(),
                    runtime_buffer_linked.as_slice().to_owned(),
                    metadata_hash,
                    metadata_json,
                ))
            }
            IR::LLVMIR(ref llvm_ir) => {
                let llvm = inkwell::context::Context::create();
                let memory_buffer =
                    inkwell::memory_buffer::MemoryBuffer::create_from_memory_range_copy(
                        llvm_ir.source.as_bytes(),
                        self.name.full_path.as_str(),
                    );
                let module = llvm
                    .create_module_from_ir(memory_buffer)
                    .map_err(|error| anyhow::anyhow!(error.to_string()))?;
                let context = era_compiler_llvm_context::EVMContext::new(
                    &llvm,
                    module,
                    llvm_options,
                    era_compiler_common::CodeSegment::Runtime,
                    optimizer,
                    Some(dependency_data),
                    debug_config,
                );
                let runtime_build = context.build(self.name.path.as_str())?;
                Ok(EVMContractBuild::new(
                    self.name,
                    identifier,
                    vec![],
                    runtime_build.as_slice().to_owned(),
                    metadata_hash,
                    metadata_json,
                ))
            }
            IR::EraVMAssembly(_) => anyhow::bail!("EraVM assembly cannot be compiled to EVM."),
        }
    }

    ///
    /// Get the list of missing deployable libraries.
    ///
    pub fn get_missing_libraries(&self) -> BTreeSet<String> {
        self.ir.get_missing_libraries()
    }
}

impl FactoryDependency for Contract {
    fn get_factory_dependencies(&self) -> HashSet<&str> {
        match self.ir {
            IR::Yul(ref yul) => yul
                .object
                .0
                .factory_dependencies
                .iter()
                .map(|path| path.as_str())
                .collect(),
            IR::EVMLA(ref evm) => evm
                .assembly
                .factory_dependencies
                .iter()
                .map(|path| path.as_str())
                .collect(),
            IR::LLVMIR(_) => HashSet::new(),
            IR::EraVMAssembly(_) => HashSet::new(),
        }
    }

    fn drain_factory_dependencies(&mut self) -> HashSet<String> {
        match self.ir {
            IR::Yul(ref mut yul) => yul.object.0.factory_dependencies.drain().collect(),
            IR::EVMLA(ref mut evm) => evm.assembly.factory_dependencies.drain().collect(),
            IR::LLVMIR(_) => HashSet::new(),
            IR::EraVMAssembly(_) => HashSet::new(),
        }
    }

    fn are_factory_dependencies_satisfied<D>(
        &self,
        evaluated_dependencies: Vec<&String>,
        resolver: &D,
    ) -> bool
    where
        D: era_compiler_llvm_context::Dependency,
    {
        match self.ir {
            IR::Yul(ref yul) => yul
                .object
                .0
                .factory_dependencies
                .iter()
                .map(|identifier| {
                    resolver
                        .resolve_path(identifier.as_str())
                        .expect("Always valid")
                })
                .all(|path| evaluated_dependencies.contains(&&path)),
            IR::EVMLA(ref evm) => evm
                .assembly
                .factory_dependencies
                .iter()
                .all(|path| evaluated_dependencies.contains(&path)),
            IR::LLVMIR(_) => true,
            IR::EraVMAssembly(_) => true,
        }
    }
}
