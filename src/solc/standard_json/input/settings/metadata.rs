//!
//! The `solc --standard-json` input settings metadata.
//!

use serde::Deserialize;
use serde::Serialize;

///
/// The `solc --standard-json` input settings metadata.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    /// The bytecode hash mode.
    pub bytecode_hash: compiler_llvm_context::MetadataHash,
}

impl Metadata {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(bytecode_hash: compiler_llvm_context::MetadataHash) -> Self {
        Self { bytecode_hash }
    }
}
