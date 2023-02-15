//!
//! The inner JSON legacy assembly code element representation.
//!

use serde::Deserialize;
use serde::Serialize;

use crate::evmla::assembly::Assembly;

///
/// The inner JSON legacy assembly code element representation.
///
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Data {
    /// The assembly code wrapper.
    Assembly(Assembly),
    /// The hash representation.
    Hash(String),
    /// The full contract path after the factory dependencies replacing pass.
    Path(String),
}

impl Data {
    ///
    /// Returns the inner assembly reference if it is present.
    ///
    pub fn get_assembly(&self) -> Option<&Assembly> {
        match self {
            Self::Assembly(ref assembly) => Some(assembly),
            Self::Hash(_) => None,
            Self::Path(_) => None,
        }
    }
    ///
    /// Returns the inner assembly mutable reference if it is present.
    ///
    pub fn get_assembly_mut(&mut self) -> Option<&mut Assembly> {
        match self {
            Self::Assembly(ref mut assembly) => Some(assembly),
            Self::Hash(_) => None,
            Self::Path(_) => None,
        }
    }

    ///
    /// Gets the contract `keccak256` hash.
    ///
    pub fn keccak256(&self) -> String {
        match self {
            Self::Assembly(assembly) => assembly.keccak256(),
            Self::Hash(hash) => panic!("Expected assembly, found hash `{hash}`"),
            Self::Path(path) => panic!("Expected assembly, found path `{path}`"),
        }
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Assembly(inner) => writeln!(f, "{inner}"),
            Self::Hash(inner) => writeln!(f, "Hash `{inner}`"),
            Self::Path(inner) => writeln!(f, "Path `{inner}`"),
        }
    }
}
