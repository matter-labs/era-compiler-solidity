//!
//! The contract Yul source code.
//!

use std::collections::HashSet;

use yul_syntax_tools::yul::parser::statement::object::Object;

use crate::yul::parser::dialect::llvm::LLVMDialect;
use crate::yul::parser::statement::object::WrappedObject;

///
/// The contract Yul source code.
///
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Yul {
    /// The Yul AST object.
    pub object: WrappedObject,
}

impl Yul {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(object: WrappedObject) -> Self {
        Self { object }
    }

    ///
    /// Extracts the runtime code from the Yul object.
    ///
    pub fn take_runtime_code(&mut self) -> Option<Object<LLVMDialect>> {
        self.object.0.inner_object.take().map(|object| *object)
    }

    ///
    /// Get the list of missing deployable libraries.
    ///
    pub fn get_missing_libraries(&self) -> HashSet<String> {
        self.object.0.get_missing_libraries()
    }
}

impl<D> era_compiler_llvm_context::EraVMWriteLLVM<D> for Yul
where
    D: era_compiler_llvm_context::Dependency,
{
    fn declare(
        &mut self,
        context: &mut era_compiler_llvm_context::EraVMContext<D>,
    ) -> anyhow::Result<()> {
        self.object.declare(context)
    }

    fn into_llvm(
        self,
        context: &mut era_compiler_llvm_context::EraVMContext<D>,
    ) -> anyhow::Result<()> {
        self.object.into_llvm(context)
    }
}

impl<D> era_compiler_llvm_context::EVMWriteLLVM<D> for Yul
where
    D: era_compiler_llvm_context::Dependency,
{
    fn declare(
        &mut self,
        context: &mut era_compiler_llvm_context::EVMContext<D>,
    ) -> anyhow::Result<()> {
        self.object.declare(context)
    }

    fn into_llvm(
        self,
        context: &mut era_compiler_llvm_context::EVMContext<D>,
    ) -> anyhow::Result<()> {
        self.object.into_llvm(context)
    }
}
