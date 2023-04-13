//!
//! The `solc --standard-json` input settings optimizer representation.
//!

use serde::Deserialize;
use serde::Serialize;

///
/// The `solc --standard-json` input settings optimizer representation.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Optimizer {
    /// Whether the optimizer is enabled.
    pub enabled: bool,
    /// The optimization mode string.
    #[serde(skip_serializing)]
    pub mode: Option<char>,
}

impl Optimizer {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(enabled: bool, mode: Option<char>) -> Self {
        Self { enabled, mode }
    }
}

impl TryFrom<&Optimizer> for compiler_llvm_context::OptimizerSettings {
    type Error = anyhow::Error;

    fn try_from(value: &Optimizer) -> Result<Self, Self::Error> {
        if let Some(mode) = value.mode {
            return Self::try_from_cli(mode);
        }

        Ok(Self::cycles())
    }
}
