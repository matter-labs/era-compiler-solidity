//!
//! The contract data.
//!

pub mod ir;
pub mod metadata;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use era_compiler_llvm_context::IContext;

use crate::build_eravm::contract::Contract as EraVMContractBuild;

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
        metadata_hash_type: era_compiler_common::MetadataHashType,
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
            era_compiler_common::MetadataHashType::None => None,
            era_compiler_common::MetadataHashType::Keccak256 => Some(
                era_compiler_common::Keccak256Hash::from_slice(metadata_json_bytes.as_slice())
                    .into(),
            ),
            era_compiler_common::MetadataHashType::IPFS => Some(
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
