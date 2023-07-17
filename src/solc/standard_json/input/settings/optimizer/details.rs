//!
//! The `solc --standard-json` input settings optimizer details.
//!

use serde::Deserialize;
use serde::Serialize;

///
/// The `solc --standard-json` input settings optimizer details.
///
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Details {
    /// Whether the constant optimizer is enabled.
    pub constant_optimizer: bool,
}

impl Details {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(constant_optimizer: bool) -> Self {
        Self { constant_optimizer }
    }
}
