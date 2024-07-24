//!
//! The contract EraVM assembly source code.
//!

///
/// The contract EraVM assembly source code.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EraVMAssembly {
    /// The EraVM assembly file path.
    pub path: String,
    /// The EraVM assembly source code.
    pub source: String,
}

impl EraVMAssembly {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(path: String, mut source: String) -> Self {
        if !source.ends_with(char::from(0)) {
            source.push(char::from(0));
        }

        Self { path, source }
    }
}
