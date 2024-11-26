//!
//! The EraVM object format.
//!

///
/// The EraVM object format.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ObjectFormat {
    /// ELF object format.
    ELF,
    /// Raw binary data.
    Raw,
}
