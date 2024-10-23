//!
//! The `solc --standard-json` input settings optimizer.
//!

///
/// The `solc --standard-json` input settings optimizer.
///
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Optimizer {
    /// The optimization mode string.
    #[serde(default = "Optimizer::default_mode")]
    pub mode: char,
    /// Whether to try to recompile with -Oz if the bytecode is too large.
    #[serde(default)]
    pub fallback_to_optimizing_for_size: bool,
}

impl Default for Optimizer {
    fn default() -> Self {
        Self {
            mode: Self::default_mode(),
            fallback_to_optimizing_for_size: Default::default(),
        }
    }
}

impl Optimizer {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(mode: char, fallback_to_optimizing_for_size: bool) -> Self {
        Self {
            mode,
            fallback_to_optimizing_for_size,
        }
    }

    ///
    /// The default optimization mode.
    ///
    fn default_mode() -> char {
        '3'
    }
}

impl TryFrom<&Optimizer> for era_compiler_llvm_context::OptimizerSettings {
    type Error = anyhow::Error;

    fn try_from(optimizer: &Optimizer) -> Result<Self, Self::Error> {
        let mut settings = Self::try_from_cli(optimizer.mode)?;
        if optimizer.fallback_to_optimizing_for_size {
            settings.enable_fallback_to_size();
        }
        Ok(settings)
    }
}
