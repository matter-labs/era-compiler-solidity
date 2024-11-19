//!
//! The linker output contract.
//!

///
/// The linker output contract.
///
#[derive(Debug, Default, serde::Serialize)]
pub struct Contract {
    /// Contract bytecode.
    pub bytecode: String,
    /// Contract bytecode hash.
    pub hash: String,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(bytecode: String, hash: String) -> Self {
        Self { bytecode, hash }
    }
}
