//!
//! The contract data.
//!

pub mod ir;
pub mod metadata;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use era_compiler_llvm_context::IContext;

use crate::build_eravm::contract::Contract as EraVMContractBuild;
use crate::build_evm::contract::object::Object as EVMContractObject;
use crate::build_evm::contract::Contract as EVMContractBuild;
use crate::yul::parser::wrapper::Wrap;

use self::ir::llvm_ir::LLVMIR;
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
        metadata_hash_type: era_compiler_common::EraVMMetadataHashType,
        append_cbor: bool,
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
        let metadata_json_bytes = serde_json::to_vec(&metadata_json).expect("Always valid");
        let metadata_bytes = match metadata_hash_type {
            era_compiler_common::EraVMMetadataHashType::None => None,
            era_compiler_common::EraVMMetadataHashType::Keccak256 => Some(
                era_compiler_common::Keccak256Hash::from_slice(metadata_json_bytes.as_slice())
                    .into(),
            ),
            era_compiler_common::EraVMMetadataHashType::IPFS => Some(
                era_compiler_common::IPFSHash::from_slice(metadata_json_bytes.as_slice()).into(),
            ),
        };

        let cbor_data = if append_cbor {
            let cbor_key = crate::r#const::SOLC_PRODUCTION_NAME.to_owned();
            let mut cbor_data = Vec::with_capacity(3);
            cbor_data.push((
                crate::r#const::DEFAULT_EXECUTABLE_NAME.to_owned(),
                crate::r#const::version().parse().expect("Always valid"),
            ));
            if let Some(ref solc_version) = solc_version {
                cbor_data.push((
                    crate::r#const::SOLC_PRODUCTION_NAME.to_owned(),
                    solc_version.default.to_owned(),
                ));
                cbor_data.push((
                    crate::r#const::SOLC_LLVM_REVISION_METADATA_TAG.to_owned(),
                    solc_version.l2_revision.to_owned(),
                ));
            };
            Some((cbor_key, cbor_data))
        } else {
            None
        };

        let build = match self.ir {
            IR::Yul(mut yul) => {
                let module = llvm.create_module(self.name.full_path.as_str());
                let mut context: era_compiler_llvm_context::EraVMContext =
                    era_compiler_llvm_context::EraVMContext::new(
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

                yul.declare(&mut context)?;
                yul.into_llvm(&mut context)
                    .map_err(|error| anyhow::anyhow!("LLVM IR generator: {error}"))?;

                context.build(
                    self.name.full_path.as_str(),
                    metadata_bytes,
                    cbor_data,
                    output_assembly,
                    false,
                )?
            }
            IR::EVMLA(mut evmla) => {
                let solc_version = solc_version
                    .clone()
                    .expect("The EVM assembly codegen cannot be executed without `solc`");

                let module = llvm.create_module(self.name.full_path.as_str());
                let mut context: era_compiler_llvm_context::EraVMContext =
                    era_compiler_llvm_context::EraVMContext::new(
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
                    metadata_bytes,
                    cbor_data,
                    output_assembly,
                    false,
                )?
            }
            IR::LLVMIR(mut llvm_ir) => {
                llvm_ir.source.push(char::from(0));
                let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
                    &llvm_ir.source.as_bytes()[..llvm_ir.source.len() - 1],
                    self.name.full_path.as_str(),
                    true,
                );

                let module = llvm
                    .create_module_from_ir(memory_buffer)
                    .map_err(|error| anyhow::anyhow!(error.to_string()))?;
                let context: era_compiler_llvm_context::EraVMContext =
                    era_compiler_llvm_context::EraVMContext::new(
                        &llvm,
                        module,
                        llvm_options,
                        optimizer,
                        debug_config,
                    );

                context.build(
                    self.name.full_path.as_str(),
                    metadata_bytes,
                    cbor_data,
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
                    metadata_bytes,
                    cbor_data,
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
        solc_version: Option<era_solc::Version>,
        identifier_paths: BTreeMap<String, String>,
        unlinked_libraries: BTreeSet<String>,
        metadata_hash_type: era_compiler_common::EVMMetadataHashType,
        optimizer_settings: era_compiler_llvm_context::OptimizerSettings,
        llvm_options: Vec<String>,
        debug_config: Option<era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<EVMContractBuild> {
        use era_compiler_llvm_context::EVMWriteLLVM;

        let identifier = self.identifier().to_owned();

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
        let metadata_json_bytes = serde_json::to_vec(&metadata_json).expect("Always valid");
        let metadata_bytes = match metadata_hash_type {
            era_compiler_common::EVMMetadataHashType::None => None,
            era_compiler_common::EVMMetadataHashType::IPFS => Some(
                era_compiler_common::IPFSHash::from_slice(metadata_json_bytes.as_slice()).to_vec(),
            ),
        };

        let deploy_code_segment = era_compiler_common::CodeSegment::Deploy;
        let runtime_code_segment = era_compiler_common::CodeSegment::Runtime;

        match self.ir {
            IR::Yul(mut deploy_code) => {
                let runtime_code = deploy_code.take_runtime_code().ok_or_else(|| {
                    anyhow::anyhow!("Contract `{identifier}` has no runtime code")
                })?;

                let deploy_code_dependecies = deploy_code.get_evm_dependencies(Some(&runtime_code));
                let runtime_code_dependecies = runtime_code.get_evm_dependencies(None);
                let mut runtime_code = runtime_code.wrap();

                let deploy_code_identifier = deploy_code.object.0.identifier.clone();
                let runtime_code_identifier = runtime_code.0.identifier.clone();

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
                    debug_config.clone(),
                );
                runtime_context.set_yul_data(era_compiler_llvm_context::EVMContextYulData::new(
                    identifier_paths.clone(),
                ));
                runtime_code.declare(&mut runtime_context)?;
                runtime_code
                    .into_llvm(&mut runtime_context)
                    .map_err(|error| {
                        anyhow::anyhow!("{runtime_code_segment} code LLVM IR generator: {error}")
                    })?;
                let runtime_build = runtime_context.build(false, true, false)?;
                let runtime_object = EVMContractObject::new(
                    runtime_code_identifier,
                    self.name.clone(),
                    runtime_build.bytecode.expect("Always exists"),
                    Some(era_solc::StandardJsonInputCodegen::Yul),
                    runtime_code_segment,
                    metadata_bytes,
                    runtime_code_dependecies,
                    unlinked_libraries.clone(),
                    era_compiler_common::ObjectFormat::ELF,
                    runtime_build.warnings,
                );

                let immutables_map = runtime_build.immutables.unwrap_or_default();

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
                    debug_config.clone(),
                );
                deploy_context.set_solidity_data(
                    era_compiler_llvm_context::EVMContextSolidityData::new(immutables_map),
                );
                deploy_context.set_yul_data(era_compiler_llvm_context::EVMContextYulData::new(
                    identifier_paths,
                ));
                deploy_code.declare(&mut deploy_context)?;
                deploy_code
                    .into_llvm(&mut deploy_context)
                    .map_err(|error| {
                        anyhow::anyhow!("{deploy_code_segment} code LLVM IR generator: {error}")
                    })?;
                let deploy_build = deploy_context.build(false, true, false)?;
                let deploy_object = EVMContractObject::new(
                    deploy_code_identifier,
                    self.name.clone(),
                    deploy_build.bytecode.expect("Always exists"),
                    Some(era_solc::StandardJsonInputCodegen::Yul),
                    deploy_code_segment,
                    None,
                    deploy_code_dependecies,
                    unlinked_libraries,
                    era_compiler_common::ObjectFormat::ELF,
                    deploy_build.warnings,
                );

                Ok(EVMContractBuild::new(
                    self.name,
                    deploy_object,
                    runtime_object,
                    metadata_json,
                ))
            }
            IR::EVMLA(mut deploy_code) => {
                let mut runtime_code_assembly = deploy_code.assembly.runtime_code()?.to_owned();
                runtime_code_assembly.set_full_path(deploy_code.assembly.full_path().to_owned());

                let deploy_code_identifier = self.name.full_path.to_owned();
                let runtime_code_identifier =
                    format!("{}.{runtime_code_segment}", self.name.full_path);

                let mut deploy_code_dependencies =
                    era_yul::Dependencies::new(deploy_code_identifier.as_str());
                deploy_code.accumulate_evm_dependencies(&mut deploy_code_dependencies);
                let mut runtime_code_dependencies =
                    era_yul::Dependencies::new(runtime_code_identifier.as_str());
                runtime_code_assembly.accumulate_evm_dependencies(&mut runtime_code_dependencies);

                let evmla_data = era_compiler_llvm_context::EVMContextEVMLAData::new(
                    solc_version.expect("Always exists").default,
                );

                let runtime_llvm = inkwell::context::Context::create();
                let runtime_module = runtime_llvm.create_module(runtime_code_identifier.as_str());
                let mut runtime_context = era_compiler_llvm_context::EVMContext::new(
                    &runtime_llvm,
                    runtime_module,
                    llvm_options.clone(),
                    runtime_code_segment,
                    optimizer.clone(),
                    debug_config.clone(),
                );
                runtime_context.set_evmla_data(evmla_data.clone());
                runtime_code_assembly.declare(&mut runtime_context)?;
                runtime_code_assembly
                    .into_llvm(&mut runtime_context)
                    .map_err(|error| {
                        anyhow::anyhow!("{runtime_code_segment} code LLVM IR generator: {error}")
                    })?;
                let runtime_build = runtime_context.build(false, true, false)?;
                let runtime_object = EVMContractObject::new(
                    runtime_code_identifier,
                    self.name.clone(),
                    runtime_build.bytecode.expect("Always exists"),
                    Some(era_solc::StandardJsonInputCodegen::EVMLA),
                    runtime_code_segment,
                    metadata_bytes,
                    runtime_code_dependencies,
                    unlinked_libraries.clone(),
                    era_compiler_common::ObjectFormat::ELF,
                    runtime_build.warnings,
                );

                let immutables_map = runtime_build.immutables.unwrap_or_default();

                let deploy_llvm = inkwell::context::Context::create();
                let deploy_module = deploy_llvm.create_module(deploy_code_identifier.as_str());
                let mut deploy_context = era_compiler_llvm_context::EVMContext::new(
                    &deploy_llvm,
                    deploy_module,
                    llvm_options.clone(),
                    deploy_code_segment,
                    optimizer.clone(),
                    debug_config.clone(),
                );
                deploy_context.set_solidity_data(
                    era_compiler_llvm_context::EVMContextSolidityData::new(immutables_map),
                );
                deploy_context.set_evmla_data(evmla_data);
                deploy_code.declare(&mut deploy_context)?;
                deploy_code
                    .into_llvm(&mut deploy_context)
                    .map_err(|error| {
                        anyhow::anyhow!("{deploy_code_segment} code LLVM IR generator: {error}")
                    })?;
                let deploy_build = deploy_context.build(false, true, false)?;
                let deploy_object = EVMContractObject::new(
                    deploy_code_identifier,
                    self.name.clone(),
                    deploy_build.bytecode.expect("Always exists"),
                    Some(era_solc::StandardJsonInputCodegen::EVMLA),
                    deploy_code_segment,
                    None,
                    deploy_code_dependencies,
                    unlinked_libraries,
                    era_compiler_common::ObjectFormat::ELF,
                    deploy_build.warnings,
                );

                Ok(EVMContractBuild::new(
                    self.name,
                    deploy_object,
                    runtime_object,
                    metadata_json,
                ))
            }
            IR::LLVMIR(mut runtime_llvm_ir) => {
                let deploy_code_identifier = self.name.full_path.to_owned();
                let runtime_code_identifier =
                    format!("{}.{runtime_code_segment}", self.name.full_path);

                let mut deploy_llvm_ir = LLVMIR::new(
                    deploy_code_identifier.clone(),
                    era_compiler_llvm_context::evm_minimal_deploy_code(
                        deploy_code_identifier.as_str(),
                        runtime_code_identifier.as_str(),
                    ),
                );
                deploy_llvm_ir.source.push(char::from(0));
                let deploy_memory_buffer =
                    inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
                        &deploy_llvm_ir.source.as_bytes()[..deploy_llvm_ir.source.len() - 1],
                        deploy_code_identifier.as_str(),
                        true,
                    );

                runtime_llvm_ir.source.push(char::from(0));
                let runtime_memory_buffer =
                    inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
                        &runtime_llvm_ir.source.as_bytes()[..runtime_llvm_ir.source.len() - 1],
                        runtime_code_identifier.as_str(),
                        true,
                    );

                let mut deploy_code_dependencies =
                    era_yul::Dependencies::new(deploy_code_identifier.as_str());
                deploy_code_dependencies.push(runtime_code_identifier.to_owned(), true);
                let runtime_code_dependencies =
                    era_yul::Dependencies::new(runtime_code_identifier.as_str());

                let runtime_llvm = inkwell::context::Context::create();
                let runtime_module = runtime_llvm
                    .create_module_from_ir(runtime_memory_buffer)
                    .map_err(|error| anyhow::anyhow!(error.to_string()))?;
                let runtime_context = era_compiler_llvm_context::EVMContext::new(
                    &runtime_llvm,
                    runtime_module,
                    llvm_options.clone(),
                    runtime_code_segment,
                    optimizer.clone(),
                    debug_config.clone(),
                );
                let runtime_build = runtime_context.build(false, true, false)?;
                let runtime_object = EVMContractObject::new(
                    runtime_code_identifier,
                    self.name.clone(),
                    runtime_build.bytecode.expect("Always exists"),
                    None,
                    runtime_code_segment,
                    metadata_bytes,
                    runtime_code_dependencies,
                    BTreeSet::new(),
                    era_compiler_common::ObjectFormat::ELF,
                    runtime_build.warnings,
                );

                let deploy_llvm = inkwell::context::Context::create();
                let deploy_module = deploy_llvm
                    .create_module_from_ir(deploy_memory_buffer)
                    .map_err(|error| anyhow::anyhow!(error.to_string()))?;
                let deploy_context = era_compiler_llvm_context::EVMContext::new(
                    &deploy_llvm,
                    deploy_module,
                    llvm_options,
                    deploy_code_segment,
                    optimizer,
                    debug_config,
                );
                let deploy_build = deploy_context.build(false, true, false)?;
                let deploy_object = EVMContractObject::new(
                    deploy_code_identifier,
                    self.name.clone(),
                    deploy_build.bytecode.expect("Always exists"),
                    None,
                    deploy_code_segment,
                    None,
                    deploy_code_dependencies,
                    BTreeSet::new(),
                    era_compiler_common::ObjectFormat::ELF,
                    deploy_build.warnings,
                );

                Ok(EVMContractBuild::new(
                    self.name,
                    deploy_object,
                    runtime_object,
                    metadata_json,
                ))
            }
            IR::EraVMAssembly(_) => anyhow::bail!("EraVM assembly cannot be compiled to EVM."),
        }
    }

    ///
    /// Get the list of missing deployable libraries.
    ///
    pub fn get_missing_libraries(&self, deployed_libraries: &BTreeSet<String>) -> BTreeSet<String> {
        self.ir
            .get_missing_libraries()
            .into_iter()
            .filter(|library| !deployed_libraries.contains(library))
            .collect::<BTreeSet<String>>()
    }
}
