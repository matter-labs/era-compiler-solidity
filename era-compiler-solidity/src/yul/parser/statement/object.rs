//!
//! The Yul object.
//!

use crate::declare_wrapper;
use crate::yul::parser::dialect::era::EraDialect;
use crate::yul::parser::wrapper::Wrap;
use era_compiler_llvm_context::IContext;

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

        for name in [
            era_compiler_llvm_context::EraVMRuntime::FUNCTION_DEPLOY_CODE,
            era_compiler_llvm_context::EraVMRuntime::FUNCTION_RUNTIME_CODE,
            era_compiler_llvm_context::EraVMRuntime::FUNCTION_ENTRY,
        ]
        .into_iter()
        {
            context
                .get_function(name)
                .expect("Always exists")
                .borrow_mut()
                .set_yul_data(era_compiler_llvm_context::EraVMFunctionYulData::default());
        }

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

impl era_compiler_llvm_context::EVMWriteLLVM for Object {
    fn declare(
        &mut self,
        _context: &mut era_compiler_llvm_context::EVMContext,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn into_llvm(self, context: &mut era_compiler_llvm_context::EVMContext) -> anyhow::Result<()> {
        let mut entry = era_compiler_llvm_context::EVMEntryFunction::new(self.0.code.wrap());
        entry.declare(context)?;
        entry.into_llvm(context)?;
        Ok(())
    }
}
