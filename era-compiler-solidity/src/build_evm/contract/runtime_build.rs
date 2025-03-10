//!
//! Runtime code.
//!

///
/// The runtime code LLVM module build.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RuntimeBuild {
    /// Object identifier.
    pub identifier: String,
    /// Bytecode.
    pub bytecode: Vec<u8>,
    /// Dependencies.
    pub dependencies: era_yul::Dependencies,
}

impl RuntimeBuild {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(identifier: String, bytecode: Vec<u8>, dependencies: era_yul::Dependencies) -> Self {
        Self {
            identifier,
            bytecode,
            dependencies,
        }
    }
}
