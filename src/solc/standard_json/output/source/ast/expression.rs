//!
//! The Solidity AST expression.
//!

use serde::Deserialize;
use serde::Serialize;

use crate::solc::standard_json::output::error::Error as SolcStandardJsonOutputError;
use crate::solc::standard_json::output::source::AST as SolcStandardJsonOutputSourceAST;

///
/// The Solidity AST expression.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Expression {
    /// The nested node.
    Node(Box<SolcStandardJsonOutputSourceAST>),
    /// Other variants.
    Other(serde_json::Value),
}

impl Expression {
    ///
    /// Returns the list of messages for some specific parts of the AST.
    ///
    pub fn get_messages(&self) -> anyhow::Result<Vec<SolcStandardJsonOutputError>> {
        match self {
            Self::Node(inner) => inner.get_messages(),
            Self::Other(_) => Ok(vec![]),
        }
    }

    ///
    /// If the expression is a node, returns the reference.
    ///
    pub fn as_node(&self) -> Option<&SolcStandardJsonOutputSourceAST> {
        match self {
            Self::Node(inner) => Some(inner),
            Self::Other(_) => None,
        }
    }
}
