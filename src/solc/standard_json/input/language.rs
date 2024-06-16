//!
//! The `solc --standard-json` input language.
//!

///
/// The `solc --standard-json` input language.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Language {
    /// Solidity language.
    Solidity,
    /// Yul IR.
    Yul,
    /// LLVM IR.
    #[serde(rename = "LLVM IR")]
    LLVMIR,
    /// EraVM assembly.
    #[serde(rename = "EraVM Assembly")]
    EraVMAssembly,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Solidity => write!(f, "Solidity"),
            Self::Yul => write!(f, "Yul"),
            Self::LLVMIR => write!(f, "LLVM IR"),
            Self::EraVMAssembly => write!(f, "EraVM Assembly"),
        }
    }
}
