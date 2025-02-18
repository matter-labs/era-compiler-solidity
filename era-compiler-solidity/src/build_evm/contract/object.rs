//!
//! Bytecode object.
//!

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
    /// Code segment.
    pub code_segment: era_compiler_common::CodeSegment,
    /// Dependencies.
    pub dependencies: era_yul::Dependencies,
    /// Whether the object is already assembled.
    pub is_assembled: bool,
    /// The binary object format.
    pub object_format: era_compiler_common::ObjectFormat,
}

impl Object {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        identifier: String,
        contract_name: era_compiler_common::ContractName,
        bytecode: Vec<u8>,
        code_segment: era_compiler_common::CodeSegment,
        dependencies: era_yul::Dependencies,
    ) -> Self {
        Self {
            identifier,
            contract_name,
            bytecode,
            code_segment,
            dependencies,
            is_assembled: false,
            object_format: era_compiler_common::ObjectFormat::ELF,
        }
    }

    ///
    /// Whether the object requires assebmling with its dependencies.
    ///
    pub fn requires_assembling(&self) -> bool {
        !self.is_assembled && !self.dependencies.inner.is_empty()
    }
}
