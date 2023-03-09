//!
//! The contract Yul source code.
//!

use crate::yul::parser::statement::object::Object;

///
/// The contract Yul source code.
///
#[derive(Debug, Clone)]
pub struct Yul {
    /// The Yul source code.
    pub source_code: String,
    /// The Yul AST object.
    pub object: Object,
}

impl Yul {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(source_code: String, object: Object) -> Self {
        Self {
            source_code,
            object,
        }
    }
}

impl<D> compiler_llvm_context::WriteLLVM<D> for Yul
where
    D: compiler_llvm_context::Dependency,
{
    fn declare(&mut self, context: &mut compiler_llvm_context::Context<D>) -> anyhow::Result<()> {
        self.object.declare(context)
    }

    fn into_llvm(self, context: &mut compiler_llvm_context::Context<D>) -> anyhow::Result<()> {
        self.object.into_llvm(context)
    }
}
