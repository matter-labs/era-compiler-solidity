//!
//! The compiler message type.
//!

use std::str::FromStr;

///
/// The compiler message type.
///
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageType {
    /// The error for eponymous feature.
    SendTransfer,

    /// The warning for eponymous feature.
    TxOrigin,
}

impl MessageType {
    ///
    /// Converts string arguments into an array of messages.
    ///
    pub fn try_from_strings(strings: &[String]) -> Result<Vec<Self>, anyhow::Error> {
        strings
            .iter()
            .map(|string| Self::from_str(string))
            .collect()
    }
}

impl FromStr for MessageType {
    type Err = anyhow::Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string {
            "sendtransfer" => Ok(Self::SendTransfer),
            "txorigin" => Ok(Self::TxOrigin),
            r#type => Err(anyhow::anyhow!("Invalid suppressed message type: {type}")),
        }
    }
}
