//!
//! The `solc --standard-json` input settings optimizer.
//!

pub mod details;

use serde::Deserialize;
use serde::Serialize;

use self::details::Details;

///
/// The `solc --standard-json` input settings optimizer.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Optimizer {
    /// Whether the optimizer is enabled.
    pub enabled: bool,
    /// The optimization mode string.
    #[serde(skip_serializing)]
    pub mode: Option<char>,
    /// The `solc` optimizer details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Details>,
}

impl Optimizer {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(enabled: bool, mode: Option<char>) -> Self {
        Self {
            enabled,
            mode,
            details: Some(Details::default()),
        }
    }

    ///
    /// Sets the necessary defaults.
    ///
    pub fn normalize(&mut self, version: &semver::Version) {
        self.details = if version >= &semver::Version::new(0, 5, 5) {
            Some(Details::default())
        } else {
            None
        };
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
