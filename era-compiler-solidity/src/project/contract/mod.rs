//!
//! The contract data.
//!

pub mod factory_dependency;
pub mod ir;
pub mod metadata;

use std::collections::HashSet;

use sha3::Digest;

use era_compiler_llvm_context::IContext;

use crate::build_eravm::contract::Contract as EraVMContractBuild;
use crate::build_evm::contract::Contract as EVMContractBuild;
use crate::process::input_eravm::dependency_data::DependencyData as EraVMProcessInputDependencyData;
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
    /// The absolute file path.
    pub path: String,
    /// The IR source code data.
    pub ir: IR,
    /// The metadata JSON.
    pub source_metadata: serde_json::Value,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(path: String, ir: IR, source_metadata: serde_json::Value) -> Self {
        Self {
            path,
            ir,
            source_metadata,
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
        mut self,
        dependency_data: EraVMProcessInputDependencyData,
        enable_eravm_extensions: bool,
        include_metadata_hash: bool,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        output_assembly: bool,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<EraVMContractBuild> {
        use era_compiler_llvm_context::EraVMWriteLLVM;

        let identifier = self.identifier().to_owned();
        let factory_dependencies = self.drain_factory_dependencies();

        let solc_version = dependency_data.solc_version.clone();

        let llvm = inkwell::context::Context::create();
        let optimizer = era_compiler_llvm_context::Optimizer::new(optimizer_settings);

        let metadata = Metadata::new(
            self.source_metadata,
            solc_version
                .as_ref()
                .map(|version| version.default.to_owned()),
            solc_version
                .as_ref()
                .and_then(|version| version.l2_revision.to_owned()),
            semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid"),
            optimizer.settings().to_owned(),
            llvm_options.as_slice(),
        );
        let metadata_json = serde_json::to_value(&metadata).expect("Always valid");
        let metadata_hash = if include_metadata_hash {
            Some(sha3::Keccak256::digest(metadata_json.to_string().as_bytes()).into())
        } else {
            None
        };

        let module = match self.ir {
            IR::LLVMIR(ref llvm_ir) => {
                let memory_buffer =
                    inkwell::memory_buffer::MemoryBuffer::create_from_memory_range_copy(
                        llvm_ir.source.as_bytes(),
                        self.path.as_str(),
                    );
                llvm.create_module_from_ir(memory_buffer)
                    .map_err(|error| anyhow::anyhow!(error.to_string()))?
            }
            IR::EraVMAssembly(eravm_assembly) => {
                let target_machine = era_compiler_llvm_context::TargetMachine::new(
                    era_compiler_common::Target::EraVM,
                    optimizer.settings(),
                    llvm_options.as_slice(),
                )?;
                let bytecode_buffer = era_compiler_llvm_context::eravm_assemble(
                    &target_machine,
                    self.path.as_str(),
                    eravm_assembly.source.as_str(),
                    debug_config.as_ref(),
                )?;
                let assembly_text = if output_assembly {
                    Some(eravm_assembly.source)
                } else {
                    None
                };
                let build = era_compiler_llvm_context::eravm_build(
                    bytecode_buffer,
                    metadata_hash,
                    assembly_text,
                )?;
                return Ok(EraVMContractBuild::new(
                    self.path,
                    identifier,
                    build,
                    metadata_json,
                    HashSet::new(),
                ));
            }
            _ => llvm.create_module(self.path.as_str()),
        };
        let mut context = era_compiler_llvm_context::EraVMContext::new(
            &llvm,
            module,
            llvm_options,
            optimizer,
            Some(dependency_data),
            debug_config,
        );
        context.set_solidity_data(era_compiler_llvm_context::EraVMContextSolidityData::default());
        match self.ir {
            IR::Yul(_) => {
                let yul_data =
                    era_compiler_llvm_context::EraVMContextYulData::new(enable_eravm_extensions);
                context.set_yul_data(yul_data);
            }
            IR::EVMLA(_) => {
                let solc_version = match solc_version {
                    Some(solc_version) => solc_version,
                    None => {
                        anyhow::bail!(
                            "The EVM assembly pipeline cannot be executed without `solc`"
                        );
                    }
                };

                let evmla_data =
                    era_compiler_llvm_context::EraVMContextEVMLAData::new(solc_version.default);
                context.set_evmla_data(evmla_data);
            }
            _ => {}
        }

        self.ir
            .declare(&mut context)
            .map_err(|error| anyhow::anyhow!("LLVM IR generator declaration pass: {error}"))?;
        self.ir
            .into_llvm(&mut context)
            .map_err(|error| anyhow::anyhow!("LLVM IR generator definition pass: {error}"))?;

        let build = context.build(self.path.as_str(), metadata_hash, output_assembly, false)?;

        Ok(EraVMContractBuild::new(
            self.path,
            identifier,
            build,
            metadata_json,
            factory_dependencies,
        ))
    }

    ///
    /// Compiles the specified contract to EVM, returning its build artifacts.
    ///
    pub fn compile_to_evm(
        self,
        dependency_data: EVMProcessInputDependencyData,
        include_metadata_hash: bool,
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
                .and_then(|version| version.l2_revision.to_owned()),
            semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid"),
            optimizer.settings().to_owned(),
            llvm_options.as_slice(),
        );
        let metadata_json = serde_json::to_value(&metadata).expect("Always valid");
        let metadata_hash = if include_metadata_hash {
            Some(sha3::Keccak256::digest(metadata_json.to_string().as_bytes()).into())
        } else {
            None
        };

        match self.ir {
            IR::Yul(mut yul) => {
                let runtime_code = yul.take_runtime_code().ok_or_else(|| {
                    anyhow::anyhow!("contract `{identifier}` has no runtime code")
                })?;
                let deploy_code = yul.object;

                let [deploy_build, runtime_build]: [anyhow::Result<
                    era_compiler_llvm_context::EVMBuild,
                >; 2] = [
                    (era_compiler_llvm_context::CodeType::Deploy, deploy_code),
                    (
                        era_compiler_llvm_context::CodeType::Runtime,
                        runtime_code.wrap(),
                    ),
                ]
                .into_iter()
                .map(|(code_type, mut code)| {
                    let llvm = inkwell::context::Context::create();
                    let module =
                        llvm.create_module(format!("{}.{}", self.path, code_type).as_str());
                    let mut context = era_compiler_llvm_context::EVMContext::new(
                        &llvm,
                        module,
                        llvm_options.clone(),
                        code_type,
                        optimizer.clone(),
                        Some(dependency_data.clone()),
                        debug_config.clone(),
                    );
                    code.declare(&mut context).map_err(|error| {
                        anyhow::anyhow!("deploy code LLVM IR generator declaration pass: {error}")
                    })?;
                    code.into_llvm(&mut context).map_err(|error| {
                        anyhow::anyhow!("deploy code LLVM IR generator definition pass: {error}")
                    })?;
                    let build = context.build(self.path.as_str(), metadata_hash)?;
                    Ok(build)
                })
                .collect::<Vec<anyhow::Result<era_compiler_llvm_context::EVMBuild>>>()
                .try_into()
                .expect("Always valid");

                Ok(EVMContractBuild::new(
                    self.path,
                    identifier,
                    deploy_build?,
                    runtime_build?,
                    metadata_json,
                ))
            }
            IR::EVMLA(evmla) => {
                let solc_version = match solc_version {
                    Some(solc_version) => solc_version,
                    None => {
                        anyhow::bail!(
                            "The EVM assembly pipeline cannot be executed without `solc`"
                        );
                    }
                };

                let mut runtime_code_assembly = evmla.assembly.get_runtime_code()?.to_owned();
                let deploy_code_assembly = evmla.assembly;
                runtime_code_assembly.set_full_path(deploy_code_assembly.full_path().to_owned());

                let [deploy_build, runtime_build]: [anyhow::Result<
                    era_compiler_llvm_context::EVMBuild,
                >; 2] = [
                    (
                        era_compiler_llvm_context::CodeType::Deploy,
                        deploy_code_assembly,
                    ),
                    (
                        era_compiler_llvm_context::CodeType::Runtime,
                        runtime_code_assembly,
                    ),
                ]
                .into_iter()
                .map(|(code_type, mut code)| {
                    let llvm = inkwell::context::Context::create();
                    let module =
                        llvm.create_module(format!("{}.{}", self.path, code_type).as_str());
                    let mut context = era_compiler_llvm_context::EVMContext::new(
                        &llvm,
                        module,
                        llvm_options.clone(),
                        code_type,
                        optimizer.clone(),
                        Some(dependency_data.clone()),
                        debug_config.clone(),
                    );
                    let evmla_data = era_compiler_llvm_context::EVMContextEVMLAData::new(
                        solc_version.default.clone(),
                    );
                    context.set_evmla_data(evmla_data);
                    code.declare(&mut context).map_err(|error| {
                        anyhow::anyhow!("deploy code LLVM IR generator declaration pass: {error}")
                    })?;
                    code.into_llvm(&mut context).map_err(|error| {
                        anyhow::anyhow!("deploy code LLVM IR generator definition pass: {error}")
                    })?;
                    let build = context.build(self.path.as_str(), metadata_hash)?;
                    Ok(build)
                })
                .collect::<Vec<anyhow::Result<era_compiler_llvm_context::EVMBuild>>>()
                .try_into()
                .expect("Always valid");

                Ok(EVMContractBuild::new(
                    self.path,
                    identifier,
                    deploy_build?,
                    runtime_build?,
                    metadata_json,
                ))
            }
            IR::LLVMIR(ref llvm_ir) => {
                let llvm = inkwell::context::Context::create();
                let memory_buffer =
                    inkwell::memory_buffer::MemoryBuffer::create_from_memory_range_copy(
                        llvm_ir.source.as_bytes(),
                        self.path.as_str(),
                    );
                let module = llvm
                    .create_module_from_ir(memory_buffer)
                    .map_err(|error| anyhow::anyhow!(error.to_string()))?;
                let context = era_compiler_llvm_context::EVMContext::new(
                    &llvm,
                    module,
                    llvm_options,
                    era_compiler_llvm_context::CodeType::Runtime,
                    optimizer,
                    Some(dependency_data.clone()),
                    debug_config,
                );
                let build = context.build(self.path.as_str(), metadata_hash)?;
                Ok(EVMContractBuild::new(
                    self.path,
                    identifier,
                    era_compiler_llvm_context::EVMBuild::default(),
                    build,
                    metadata_json,
                ))
            }
            IR::EraVMAssembly(_) => {
                anyhow::bail!("EraVM assembly cannot be compiled to the EVM target")
            }
        }
    }

    ///
    /// Get the list of missing deployable libraries.
    ///
    pub fn get_missing_libraries(&self) -> HashSet<String> {
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
