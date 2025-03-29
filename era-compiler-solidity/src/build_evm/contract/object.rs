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
    /// Codegen.
    pub codegen: Option<era_solc::StandardJsonInputCodegen>,
    /// Code segment.
    pub code_segment: era_compiler_common::CodeSegment,
    /// Dependencies.
    pub dependencies: era_yul::Dependencies,
    /// Whether the object is already assembled.
    pub is_assembled: bool,
    /// The binary object format.
    pub object_format: era_compiler_common::ObjectFormat,
    /// Warnings produced during compilation.
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
        warnings: Vec<era_compiler_llvm_context::EVMWarning>,
    ) -> Self {
        Self {
            identifier,
            contract_name,
            bytecode,
            codegen,
            code_segment,
            dependencies,
            is_assembled: false,
            object_format: era_compiler_common::ObjectFormat::ELF,
            warnings,
        }
    }

    ///
    /// Whether the object requires assebmling with its dependencies.
    ///
    pub fn requires_assembling(&self) -> bool {
        !self.is_assembled && !self.dependencies.inner.is_empty()
    }

    ///
    /// Checks whether the object name matches a dot-separated dependency name.
    ///
    /// This function is only useful for Yul codegen where object names like `A_25.A_25_deployed` are found.
    /// For EVM assembly codegen, it performs a simple comparison.
    ///
    pub fn matches_dependency(&self, dependency: &str) -> bool {
        let dependency = match self.codegen {
            Some(era_solc::StandardJsonInputCodegen::EVMLA) | None => dependency,
            Some(era_solc::StandardJsonInputCodegen::Yul) => {
                dependency.split('.').last().expect("Always exists")
            }
        };

        self.identifier.as_str() == dependency
    }
}
