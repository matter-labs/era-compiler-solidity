//!
//! The `solc --standard-json` output contract EVM data.
//!

pub mod bytecode;

use std::collections::BTreeMap;

use serde::Deserialize;
use serde::Serialize;

use crate::evmla::assembly::Assembly;

use self::bytecode::Bytecode;

///
/// The `solc --standard-json` output contract EVM data.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EVM {
    /// The contract EVM legacy assembly code.
    #[serde(rename = "legacyAssembly")]
    pub assembly: Option<Assembly>,
    /// The contract zkEVM assembly code.
    #[serde(rename = "assembly")]
    pub assembly_text: Option<String>,
    /// The contract bytecode.
    /// Is reset by that of zkEVM before yielding the compiled project artifacts.
    pub bytecode: Option<Bytecode>,
    /// The contract function signatures representation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method_identifiers: Option<BTreeMap<String, String>>,
}

impl EVM {
    ///
    /// A shortcut constructor for the zkEVM bytecode.
    ///
    pub fn new(
        assembly_text: String,
        bytecode: String,
        method_identifiers: Option<BTreeMap<String, String>>,
    ) -> Self {
        Self {
            assembly: None,
            assembly_text: Some(assembly_text),
            bytecode: Some(Bytecode::new(bytecode)),
            method_identifiers,
        }
    }
}
