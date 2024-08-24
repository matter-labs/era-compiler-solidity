//!
//! Builder for a path inside YUL AST.
//!

use crate::util::counter::Counter;

use super::step::LexicalScope;
use super::tracker::PathTracker;
use super::Path;

///
/// Facilitates building an instance of [`Path`].
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Builder {
    elements: Path,
    block_counter: Counter,
    code_counter: Counter,
    if_counter: Counter,
    for_counter: Counter,
}

impl Builder {
    ///
    /// Create a new instance of the path builder.
    ///
    pub fn new(elements: Path) -> Self {
        Self {
            elements,
            block_counter: Counter::new(),
            code_counter: Counter::new(),
            if_counter: Counter::new(),
            for_counter: Counter::new(),
        }
    }

    fn push(&mut self, step: LexicalScope) {
        self.elements.stack.push(step)
    }
}

impl PathTracker for Builder {
    fn here(&self) -> &Path {
        &self.elements
    }

    fn leave(&mut self) {
        self.elements.stack.pop();
    }

    fn enter_block(&mut self) {
        self.push(LexicalScope::Block(u32::from(self.block_counter)));
        self.block_counter.increment();
    }

    fn enter_function(&mut self, ident: &str) {
        self.push(LexicalScope::Function(ident.to_string()));
        self.block_counter.increment();
    }

    fn enter_code(&mut self) {
        self.push(LexicalScope::Code(u32::from(self.code_counter)));
        self.code_counter.increment();
    }

    fn enter_object(&mut self, identifier: &str) {
        self.push(LexicalScope::Module(String::from(identifier)));
    }

    fn enter_if_cond(&mut self) {
        self.if_counter.increment();
        self.push(LexicalScope::IfCondition(u32::from(self.if_counter)));
    }

    fn enter_if_then(&mut self) {
        self.push(LexicalScope::IfBlock(u32::from(self.if_counter)));
    }

    fn enter_for1(&mut self) {
        self.for_counter.increment();
        self.push(LexicalScope::For1(u32::from(self.for_counter)));
    }

    fn enter_for2(&mut self) {
        self.for_counter.increment();
        self.push(LexicalScope::For2(u32::from(self.for_counter)));
    }

    fn enter_for3(&mut self) {
        self.for_counter.increment();
        self.push(LexicalScope::For3(u32::from(self.for_counter)));
    }
}
