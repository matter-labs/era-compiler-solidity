//!
//! Process for compiling a single compilation unit.
//!
//! The EraVM output data.
//!

use serde::Deserialize;
use serde::Serialize;

use crate::build_eravm::contract::Contract as EraVMContractBuild;

///
/// The EraVM output data.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    /// The contract build.
    pub build: EraVMContractBuild,
}

impl Output {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(build: EraVMContractBuild) -> Self {
        Self { build }
    }
}
