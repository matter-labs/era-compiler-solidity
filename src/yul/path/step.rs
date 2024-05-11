//!
//! A piece of a path inside YUL AST.
//!

type CodeID = u32;
type BlockID = u32;
type IfID = u32;
type ForID = u32;
type WhileID = u32;
type Name = String;

/// Types of lexical blocks that are accounted for in a [`crate::yul::path::Path`] from the root
/// of YUL syntax tree to some location in it.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum LexicalBlock {
    Module(Name),
    Function(Name),
    Code(CodeID),
    Block(BlockID),
    IfCondition(IfID),
    IfBlock(IfID),
    For1(ForID),
    For2(ForID),
    For3(ForID),
    WhileCondition(WhileID),
    WhileBody(WhileID),
}

impl LexicalBlock {
    /// Function [`crate::yul::path::Path::full()`] transforms
    /// [`crate::yul::path::Path`] into a prefix for a variable name. Each
    /// [`LexicalBlock`] contributes a part to this prefix.
    pub fn full_name_contribution(&self) -> String {
        match self {
            LexicalBlock::Module(name) => name.to_string(),
            LexicalBlock::Function(name) => name.to_string(),
            LexicalBlock::Code(id) => format!("code{id}"),
            LexicalBlock::Block(id) => format!("block{id}"),
            LexicalBlock::IfCondition(id) => format!("if_{id}_cond"),
            LexicalBlock::IfBlock(id) => format!("if_{id}_true"),
            LexicalBlock::For1(id) => format!("for_{id}_1"),
            LexicalBlock::For2(id) => format!("for_{id}_2"),
            LexicalBlock::For3(id) => format!("for_{id}_3"),
            LexicalBlock::WhileCondition(id) => format!("while_{id}_cond"),
            LexicalBlock::WhileBody(id) => format!("while_{id}_body"),
        }
    }
}
