//!
//! The contract Yul source code.
//!

use std::collections::HashSet;

use era_yul::yul::lexer::Lexer;
use era_yul::yul::parser::statement::object::Object;

use crate::yul::parser::dialect::era::EraDialect;
use crate::yul::parser::wrapper::Wrap;

///
/// The contract Yul source code.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Yul {
    /// The Yul AST object.
    pub object: crate::yul::parser::statement::object::Object,
}

impl Yul {
    ///
    /// Transforms the `solc` standard JSON output contract into a Yul object.
    ///
    pub fn try_from_source(
        name: &era_compiler_common::ContractName,
        source_code: Option<&str>,
        debug_config: Option<&era_compiler_llvm_context::DebugConfig>,
    ) -> anyhow::Result<Option<Self>> {
        let source_code = match source_code {
            None | Some("") => return Ok(None),
            Some(ir_optimized) => ir_optimized,
        };

        if let Some(debug_config) = debug_config {
            debug_config.dump_yul(name.full_path.as_str(), None, source_code)?;
        }

        let mut lexer = Lexer::new(source_code.to_owned());
        let object = Object::parse(&mut lexer, None)
            .map_err(|error| anyhow::anyhow!("Yul parsing: {error:?}"))?;

        Ok(Some(Self {
            object: object.wrap(),
        }))
    }

    ///
    /// Extracts the runtime code from the Yul object.
    ///
    pub fn take_runtime_code(&mut self) -> Option<Object<EraDialect>> {
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
    D: era_compiler_llvm_context::Dependency + Clone,
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
    D: era_compiler_llvm_context::Dependency + Clone,
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
