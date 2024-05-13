//!
//! Builder for a path inside YUL AST.
//!

use crate::util::counter::Counter;

use super::step::LexicalBlock;
use super::Path;

/// Facilitates building an instance of [`Path`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Builder {
    elements: Path,
    block_counter: Counter,
    code_counter: Counter,
    if_counter: Counter,
    for_counter: Counter,
}

impl Builder {
    /// Create a new instance of the path builder.
    pub fn new() -> Self {
        Self {
            elements: Path { stack: [].to_vec() },
            block_counter: Counter::new(),
            code_counter: Counter::new(),
            if_counter: Counter::new(),
            for_counter: Counter::new(),
        }
    }

    /// Currently constructed path from the root of YUL syntax tree.
    pub fn here(&self) -> &Path {
        &self.elements
    }

    fn push(&mut self, step: LexicalBlock) {
        self.elements.stack.push(step)
    }

    /// Exit the last lexical block on the way from the root of YUL syntax tree.
    pub fn leave(&mut self) {
        self.elements.stack.pop();
    }

    /// Enter a block of statements between curly braces on the way from the root of YUL syntax tree.
    pub fn enter_block(&mut self) {
        self.push(LexicalBlock::Block(u32::from(self.block_counter)));
        self.block_counter.increment();
    }

    /// Enter a function on the way from the root of YUL syntax tree.
    pub fn enter_function(&mut self, ident: String) {
        self.push(LexicalBlock::Function(ident));
        self.block_counter.increment();
    }

    /// Enter a code section on the way from the root of YUL syntax tree.
    pub fn enter_code(&mut self) {
        self.push(LexicalBlock::Code(u32::from(self.code_counter)));
        self.code_counter.increment();
    }

    /// Enter a YUL object section on the way from the root of YUL syntax tree.
    pub fn enter_object(&mut self, identifier: &str) {
        self.push(LexicalBlock::Module(String::from(identifier)));
    }

    /// Enter the condition of an "if" statement on the way from the root of YUL syntax tree.
    pub fn enter_if_cond(&mut self) {
        self.if_counter.increment();
        self.push(LexicalBlock::IfCondition(u32::from(self.if_counter)));
    }

    /// Enter the "yes" branch of an "if" statement on the way from the root of YUL syntax tree.
    pub fn enter_if_then(&mut self) {
        self.push(LexicalBlock::IfBlock(u32::from(self.if_counter)));
    }
}