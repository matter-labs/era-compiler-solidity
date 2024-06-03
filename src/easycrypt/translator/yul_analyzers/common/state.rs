//!
//! State for a generic analysis pass.
//!

use crate::easycrypt::translator::definition_info::DefinitionInfo;
use crate::yul::path::symbol_table::SymbolTable;

/// State of a pass.
pub struct State<'a> {
    /// Reference to a table describing all definitions in YUL code.
    pub symbol_table: &'a mut SymbolTable<DefinitionInfo>,
}

impl<'a> State<'a> {
    /// Returns a new instance.
    pub fn new(symbol_table: &'a mut SymbolTable<DefinitionInfo>) -> Self {
        Self { symbol_table }
    }
}
