//!
//! The `solc --standard-json` output source.
//!

pub mod ast;

use serde::Deserialize;
use serde::Serialize;

use self::ast::AST;

///
/// The `solc --standard-json` output source.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    /// The source code ID.
    pub id: usize,
    /// The source code AST.
    pub ast: Option<AST>,
}
