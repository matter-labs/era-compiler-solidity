//!
//! The Solidity compiler pipeline type.
//!

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
