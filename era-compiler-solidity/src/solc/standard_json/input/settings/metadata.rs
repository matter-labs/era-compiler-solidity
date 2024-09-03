//!
//! The `solc --standard-json` input settings metadata.
//!

///
/// The `solc --standard-json` input settings metadata.
///
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    /// Whether to use literal content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_literal_content: Option<bool>,

    /// The bytecode hash mode.
    #[serde(skip_serializing)]
    pub bytecode_hash: Option<era_compiler_common::HashType>,
}

impl Metadata {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(bytecode_hash: era_compiler_common::HashType, use_literal_content: bool) -> Self {
        Self {
            bytecode_hash: Some(bytecode_hash),
            use_literal_content: Some(use_literal_content),
        }
    }
}
