//!
//! The `solc --standard-json` input settings optimizer.
//!

pub mod details;

use crate::solc::Compiler as SolcCompiler;

use self::details::Details;

///
/// The `solc --standard-json` input settings optimizer.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Optimizer {
    /// Whether the optimizer is enabled.
    pub enabled: bool,
    /// The `solc` optimizer details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Details>,

    /// The optimization mode string.
    #[serde(skip_serializing)]
    pub mode: Option<char>,
    /// Whether to try to recompile with -Oz if the bytecode is too large.
    #[serde(skip_serializing)]
    pub fallback_to_optimizing_for_size: Option<bool>,
}

impl Optimizer {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        enabled: bool,
        mode: Option<char>,
        version: &semver::Version,
        fallback_to_optimizing_for_size: bool,
    ) -> Self {
        Self {
            enabled,
            mode,
            details: if version >= &semver::Version::new(0, 5, 5) {
                Some(Details::disabled(version))
            } else {
                None
            },
            fallback_to_optimizing_for_size: Some(fallback_to_optimizing_for_size),
        }
    }

    ///
    /// A shortcut constructor for Yul validation.
    ///
    pub fn new_yul_validation() -> Self {
        Self::new(true, None, &SolcCompiler::LAST_SUPPORTED_VERSION, false)
    }

    ///
    /// Sets the necessary defaults.
    ///
    pub fn normalize(&mut self, version: &semver::Version) {
        self.details = if version >= &semver::Version::new(0, 5, 5) {
            Some(Details::disabled(version))
        } else {
            None
        };
    }
}

impl TryFrom<&Optimizer> for era_compiler_llvm_context::OptimizerSettings {
    type Error = anyhow::Error;

    fn try_from(value: &Optimizer) -> Result<Self, Self::Error> {
        let mut result = match value.mode {
            Some(mode) => Self::try_from_cli(mode)?,
            None => Self::cycles(),
        };
        if value.fallback_to_optimizing_for_size.unwrap_or_default() {
            result.enable_fallback_to_size();
        }
        Ok(result)
    }
}
