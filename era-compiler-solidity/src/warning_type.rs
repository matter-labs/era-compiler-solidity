//!
//! The compiler warning type.
//!

use std::str::FromStr;

///
/// The compiler warning type.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WarningType {
    /// The eponymous feature.
    TxOrigin,
}

impl WarningType {
    ///
    /// Converts string arguments into an array of warnings.
    ///
    pub fn try_from_strings(strings: &[String]) -> Result<Vec<Self>, anyhow::Error> {
        strings
            .iter()
            .map(|string| Self::from_str(string))
            .collect()
    }
}

impl FromStr for WarningType {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "txorigin" => Ok(Self::TxOrigin),
            r#type => Err(anyhow::anyhow!("Invalid suppressed warning type: {type}")),
        }
    }
}
