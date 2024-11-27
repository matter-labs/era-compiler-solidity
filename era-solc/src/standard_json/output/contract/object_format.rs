//!
//! The binary object format.
//!

///
/// The binary object format.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ObjectFormat {
    /// ELF object format.
    ELF,
    /// Raw binary data.
    Raw,
}
