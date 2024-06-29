//!
//! The contract Yul source code.
//!

use std::collections::HashSet;

use crate::yul::parser::statement::object::Object;

///
/// The contract Yul source code.
///
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
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

    ///
    /// Extracts the runtime code from the Yul object.
    ///
    pub fn take_runtime_code(&mut self) -> Option<Object> {
        self.object.inner_object.take().map(|object| *object)
    }

    ///
    /// Get the list of missing deployable libraries.
    ///
    pub fn get_missing_libraries(&self) -> HashSet<String> {
        self.object.get_missing_libraries()
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
