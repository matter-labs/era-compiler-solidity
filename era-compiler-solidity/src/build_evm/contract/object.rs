//!
//! Bytecode object.
//!

use std::collections::BTreeMap;
use std::collections::BTreeSet;

///
/// Bytecode object.
///
/// Can be either deploy and runtime code.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Object {
    /// Object identifier.
    pub identifier: String,
    /// Contract full name.
    pub contract_name: era_compiler_common::ContractName,
    /// Bytecode.
    pub bytecode: Vec<u8>,
    /// Codegen.
    pub codegen: Option<era_solc::StandardJsonInputCodegen>,
    /// Code segment.
    pub code_segment: era_compiler_common::CodeSegment,
    /// Dependencies.
    pub dependencies: era_yul::Dependencies,
    /// The unlinked unlinked libraries.
    pub unlinked_libraries: BTreeSet<String>,
    /// Whether the object is already assembled.
    pub is_assembled: bool,
    /// Binary object format.
    pub format: era_compiler_common::ObjectFormat,
    /// Compilation warnings.
    pub warnings: Vec<era_compiler_llvm_context::EVMWarning>,
}

impl Object {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        identifier: String,
        contract_name: era_compiler_common::ContractName,
        bytecode: Vec<u8>,
        codegen: Option<era_solc::StandardJsonInputCodegen>,
        code_segment: era_compiler_common::CodeSegment,
        dependencies: era_yul::Dependencies,
        unlinked_libraries: BTreeSet<String>,
        format: era_compiler_common::ObjectFormat,
        warnings: Vec<era_compiler_llvm_context::EVMWarning>,
    ) -> Self {
        Self {
            identifier,
            contract_name,
            bytecode,
            codegen,
            code_segment,
            dependencies,
            unlinked_libraries,
            is_assembled: false,
            format,
            warnings,
        }
    }

    ///
    /// Links the object with its linker symbols.
    ///
    pub fn link(
        &mut self,
        linker_symbols: &BTreeMap<String, [u8; era_compiler_common::BYTE_LENGTH_ETH_ADDRESS]>,
    ) -> anyhow::Result<()> {
        let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
            self.bytecode.as_slice(),
            self.identifier.as_str(),
            false,
        );

        let (linked_object, object_format) =
            era_compiler_llvm_context::evm_link(memory_buffer, linker_symbols)?;
        self.format = object_format;

        self.bytecode = linked_object.as_slice().to_owned();
        // if let era_compiler_common::CodeSegment::Deploy = self.code_segment {
        //     let metadata = match contract.metadata_hash {
        //         Some(era_compiler_common::Hash::IPFS(ref hash)) => {
        //             let cbor = era_compiler_common::CBOR::new(
        //                 Some((
        //                     era_compiler_common::EVMMetadataHashType::IPFS,
        //                     hash.as_bytes(),
        //                 )),
        //                 crate::r#const::SOLC_PRODUCTION_NAME.to_owned(),
        //                 cbor_data.clone(),
        //             );
        //             cbor.to_vec()
        //         }
        //         Some(era_compiler_common::Hash::Keccak256(ref hash)) => hash.to_vec(),
        //         None => {
        //             let cbor = era_compiler_common::CBOR::<'_, String>::new(
        //                 None,
        //                 crate::r#const::SOLC_PRODUCTION_NAME.to_owned(),
        //                 cbor_data.clone(),
        //             );
        //             cbor.to_vec()
        //         }
        //     };
        //     self.bytecode.extend(metadata);
        // }
        Ok(())
    }

    ///
    /// Whether the object requires assebmling with its dependencies.
    ///
    pub fn requires_assembling(&self) -> bool {
        !self.is_assembled && !self.dependencies.inner.is_empty()
    }
}
