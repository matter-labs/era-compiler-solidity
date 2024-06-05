//!
//! The contract data.
//!

pub mod ir;
pub mod metadata;

use std::collections::HashSet;

use serde::Deserialize;
use serde::Serialize;
use sha3::Digest;

use era_compiler_llvm_context::IContext;

use crate::build_eravm::contract::Contract as EraVMContractBuild;
use crate::build_evm::contract::Contract as EVMContractBuild;
use crate::project::Project;
use crate::solc::version::Version as SolcVersion;

use self::ir::IR;
use self::metadata::Metadata;

///
/// The contract data.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Contract {
    /// The absolute file path.
    pub path: String,
    /// The IR source code data.
    pub ir: IR,
    /// The metadata JSON.
    pub metadata_json: serde_json::Value,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        path: String,
        ir: IR,
        metadata_json: Option<serde_json::Value>,
        hash: [u8; era_compiler_common::BYTE_LENGTH_FIELD],
        solc_version: Option<&SolcVersion>,
    ) -> Self {
        let hash = hex::encode(hash.as_slice());
        let solc_version = solc_version
            .as_ref()
            .map(|solc_version| serde_json::to_value(solc_version).expect("Always valid"))
            .unwrap_or(serde_json::Value::Null);
        let metadata_json = metadata_json.unwrap_or_else(|| {
            serde_json::json!({
                "solc_version": solc_version,
                "hash": hash,
            })
        });

        Self {
            path,
            ir,
            metadata_json,
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
            IR::ZKASM(ref zkasm) => zkasm.path.as_str(),
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
            IR::ZKASM(_) => HashSet::new(),
        }
    }

    ///
    /// Compiles the specified contract to EraVM, returning its build artifacts.
    ///
    pub fn compile_to_eravm(
        mut self,
        project: Project,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: &[String],
        is_system_mode: bool,
        include_metadata_hash: bool,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<EraVMContractBuild> {
        use era_compiler_llvm_context::EraVMWriteLLVM;

        let llvm = inkwell::context::Context::create();
        let optimizer = era_compiler_llvm_context::Optimizer::new(optimizer_settings);

        let identifier = self.identifier().to_owned();
        let solc_version = project.solc_version.clone();

        let metadata = Metadata::new(
            self.metadata_json.take(),
            solc_version
                .as_ref()
                .map(|version| version.default.to_owned()),
            solc_version
                .as_ref()
                .and_then(|version| version.l2_revision.to_owned()),
            semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid"),
            optimizer.settings().to_owned(),
        );
        let metadata_json = serde_json::to_value(&metadata).expect("Always valid");
        let metadata_hash: Option<[u8; era_compiler_common::BYTE_LENGTH_FIELD]> =
            if include_metadata_hash {
                let metadata_string = serde_json::to_string(&metadata).expect("Always valid");
                Some(sha3::Keccak256::digest(metadata_string.as_bytes()).into())
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
            IR::ZKASM(ref zkasm) => {
                let build = era_compiler_llvm_context::eravm_build_assembly_text(
                    self.path.as_str(),
                    zkasm.source.as_str(),
                    metadata_hash,
                    debug_config.as_ref(),
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
            llvm_options.to_owned(),
            optimizer,
            Some(project),
            include_metadata_hash,
            debug_config,
        );
        context.set_solidity_data(era_compiler_llvm_context::EraVMContextSolidityData::default());
        match self.ir {
            IR::Yul(_) => {
                let yul_data = era_compiler_llvm_context::EraVMContextYulData::new(is_system_mode);
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

        let factory_dependencies = self.drain_factory_dependencies();

        self.ir
            .declare(&mut context)
            .map_err(|error| anyhow::anyhow!("LLVM IR generator declaration pass: {error}"))?;
        self.ir
            .into_llvm(&mut context)
            .map_err(|error| anyhow::anyhow!("LLVM IR generator definition pass: {error}"))?;

        let build = context.build(self.path.as_str(), metadata_hash)?;

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
        mut self,
        project: Project,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: &[String],
        include_metadata_hash: bool,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<EVMContractBuild> {
        use era_compiler_llvm_context::EVMWriteLLVM;

        let optimizer = era_compiler_llvm_context::Optimizer::new(optimizer_settings);

        let solc_version = project.solc_version.clone();

        let metadata = Metadata::new(
            self.metadata_json.take(),
            solc_version
                .as_ref()
                .map(|version| version.default.to_owned()),
            solc_version
                .as_ref()
                .and_then(|version| version.l2_revision.to_owned()),
            semver::Version::parse(env!("CARGO_PKG_VERSION")).expect("Always valid"),
            optimizer.settings().to_owned(),
        );
        let metadata_json = serde_json::to_value(&metadata).expect("Always valid");
        let metadata_hash: Option<[u8; era_compiler_common::BYTE_LENGTH_FIELD]> =
            if include_metadata_hash {
                let metadata_string = serde_json::to_string(&metadata).expect("Always valid");
                Some(sha3::Keccak256::digest(metadata_string.as_bytes()).into())
            } else {
                None
            };

        let identifier = self.identifier().to_owned();

        match self.ir {
            IR::Yul(mut yul) => {
                let runtime_code = yul.take_runtime_code().ok_or_else(|| {
                    anyhow::anyhow!("Contract `{identifier}` has no runtime code")
                })?;
                let deploy_code = yul.object;

                let [deploy_build, runtime_build]: [anyhow::Result<
                    era_compiler_llvm_context::EVMBuild,
                >; 2] = [
                    (era_compiler_llvm_context::CodeType::Deploy, deploy_code),
                    (era_compiler_llvm_context::CodeType::Runtime, runtime_code),
                ]
                .into_iter()
                .map(|(code_type, mut code)| {
                    let llvm = inkwell::context::Context::create();
                    let module =
                        llvm.create_module(format!("{}.{}", self.path, code_type).as_str());
                    let mut context = era_compiler_llvm_context::EVMContext::new(
                        &llvm,
                        module,
                        llvm_options.to_owned(),
                        code_type,
                        optimizer.clone(),
                        Some(project.clone()),
                        include_metadata_hash,
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
                        llvm_options.to_owned(),
                        code_type,
                        optimizer.clone(),
                        Some(project.clone()),
                        include_metadata_hash,
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
                    llvm_options.to_owned(),
                    era_compiler_llvm_context::CodeType::Runtime,
                    optimizer,
                    Some(project),
                    include_metadata_hash,
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
            IR::ZKASM(_) => anyhow::bail!("EraVM assembly cannot be compiled to the EVM target"),
        }
    }

    ///
    /// Get the list of missing deployable libraries.
    ///
    pub fn get_missing_libraries(&self) -> HashSet<String> {
        self.ir.get_missing_libraries()
    }
}
