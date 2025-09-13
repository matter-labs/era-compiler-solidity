//!
//! The `solc --standard-json` input settings metadata.
//!

///
/// The `solc --standard-json` input settings metadata.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    /// Whether to use literal content.
    #[serde(default)]
    pub use_literal_content: bool,

    /// Whether to append CBOR metadata.
    #[serde(
        rename = "appendCBOR",
        default = "Metadata::default_append_cbor",
        skip_serializing
    )]
    pub append_cbor: bool,

    /// The metadata hash type.
    #[serde(
        alias = "bytecodeHash",
        default = "Metadata::default_hash_type",
        skip_serializing
    )]
    pub hash_type: String,
}

impl Default for Metadata {
    fn default() -> Self {
        Self::new(false, true, Self::default_hash_type())
    }
}

impl Metadata {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(use_literal_content: bool, append_cbor: bool, hash_type: String) -> Self {
        Self {
            use_literal_content,
            append_cbor,

            hash_type,
        }
    }

    ///
    /// The default metadata hash type.
    ///
    fn default_hash_type() -> String {
        era_compiler_common::MetadataHashType::IPFS.to_string()
    }

    ///
    /// The default append CBOR flag.
    ///
    fn default_append_cbor() -> bool {
        true
    }
}
