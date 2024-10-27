//!
//! The `solc --standard-json` output contract EraVM data.
//!

///
/// The `solc --standard-json` output contract EraVM data.
///
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EraVM {
    /// The contract bytecode.
    pub bytecode: String,
    /// The contract text assembly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assembly: Option<String>,
}

impl EraVM {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(bytecode: String, assembly: Option<String>) -> Self {
        Self { bytecode, assembly }
    }
}
