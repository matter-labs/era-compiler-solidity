//!
//! The Solidity compiler pipeline type.
//!

use crate::solc::version::Version as SolcVersion;
use crate::solc::Compiler as SolcCompiler;

///
/// The Solidity compiler pipeline type.
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pipeline {
    /// The Yul IR.
    Yul,
    /// The EVM legacy assembly IR.
    EVMLA,
}

impl Pipeline {
    ///
    /// We always use EVMLA for Solidity <=0.7, or if the user does not want to compile via Yul.
    ///
    pub fn new(solc_version: &SolcVersion, force_evmla: bool) -> Self {
        if solc_version.default < SolcCompiler::FIRST_YUL_VERSION || force_evmla {
            Self::EVMLA
        } else {
            Self::Yul
        }
    }
}
