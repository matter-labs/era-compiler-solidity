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
    /// With the upstream `solc`, we always use EVMLA for Solidity <=0.7.
    /// With the ZKsync fork of `solc`, the default `solc` settings are used.
    ///
    /// Thus, `force_evmla` is used to switch to the old codegen with the upstream `solc`,
    /// and `via_ir` is used to switch to the new codegen with the ZKsync fork of `solc`.
    ///
    pub fn new(solc_version: &SolcVersion, force_evmla: bool, via_ir: bool) -> Self {
        match solc_version.l2_revision {
            Some(_) if via_ir => Self::Yul,
            Some(_) => Self::EVMLA,
            None if solc_version.default < SolcCompiler::FIRST_YUL_VERSION || force_evmla => {
                Self::EVMLA
            }
            None => Self::Yul,
        }
    }
}
