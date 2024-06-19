//!
//! EasyCrypt AST node containing a block of statements.
//!

use crate::easycrypt::syntax::statement::Statement;

/// EasyCrypt AST node containing a block of statements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    /// Body of the block.
    pub statements: Vec<Statement>,
}

impl Default for Block {
    fn default() -> Self {
        Self::new()
    }
}

impl Block {
    /// Create a new, empty instance of a block of statements.
    pub fn new() -> Self {
        Self { statements: vec![] }
    }
}
