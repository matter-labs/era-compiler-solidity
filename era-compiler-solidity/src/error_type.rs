//!
//! The compiler error type.
//!

use std::str::FromStr;

///
/// The compiler error type.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ErrorType {
    /// The eponymous feature.
    SendTransfer,
}

impl ErrorType {
    ///
    /// Converts string arguments into an array of errors.
    ///
    pub fn try_from_strings(strings: &[String]) -> Result<Vec<Self>, anyhow::Error> {
        strings
            .iter()
            .map(|string| Self::from_str(string))
            .collect()
    }
}

impl FromStr for ErrorType {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "sendtransfer" => Ok(Self::SendTransfer),
            r#type => Err(anyhow::anyhow!("Invalid suppressed error type: {type}")),
        }
    }
}

impl std::fmt::Display for ErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::SendTransfer => write!(f, "sendtransfer"),
        }
    }
}
