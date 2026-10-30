//!
//! The Yul object.
//!

use crate::declare_wrapper;
use crate::yul::parser::dialect::era::EraDialect;
use crate::yul::parser::wrapper::Wrap;

declare_wrapper!(
    era_yul::yul::parser::statement::object::Object<EraDialect>,
    Object
);

impl era_compiler_llvm_context::EraVMWriteLLVM for Object {
    fn declare(
        &mut self,
        context: &mut era_compiler_llvm_context::EraVMContext,
    ) -> anyhow::Result<()> {
        let mut entry = era_compiler_llvm_context::EraVMEntryFunction::default();
        entry.declare(context)?;

        let mut runtime = era_compiler_llvm_context::EraVMRuntime::default();
        runtime.declare(context)?;

        era_compiler_llvm_context::EraVMDeployCodeFunction::new(
            era_compiler_llvm_context::EraVMDummyLLVMWritable::default(),
        )
        .declare(context)?;
        era_compiler_llvm_context::EraVMRuntimeCodeFunction::new(
            era_compiler_llvm_context::EraVMDummyLLVMWritable::default(),
        )
        .declare(context)?;

        entry.into_llvm(context)?;

        Ok(())
    }

    fn into_llvm(
        self,
        context: &mut era_compiler_llvm_context::EraVMContext,
    ) -> anyhow::Result<()> {
        let term = self.0;
        if term.identifier.ends_with("_deployed") {
            era_compiler_llvm_context::EraVMRuntimeCodeFunction::new(term.code.wrap())
                .into_llvm(context)?;
        } else {
            era_compiler_llvm_context::EraVMDeployCodeFunction::new(term.code.wrap())
                .into_llvm(context)?;
        }

        match term.inner_object {
            Some(object) => {
                Object(*object).into_llvm(context)?;
            }
            None => {
                let runtime = era_compiler_llvm_context::EraVMRuntime::default();
                runtime.into_llvm(context)?;
            }
        }

        Ok(())
    }
}
