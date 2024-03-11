//!
//! Process for compiling a single compilation unit.
//!
//! The EVM output data.
//!

use serde::Deserialize;
use serde::Serialize;

use crate::build_evm::contract::Contract as EVMContractBuild;

///
/// The EVM output data.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    /// The contract build.
    pub build: EVMContractBuild,
}

impl Output {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(build: EVMContractBuild) -> Self {
        Self { build }
    }
}
