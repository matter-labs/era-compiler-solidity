//!
//! The YUL object.
//!

use crate::create_wrapper;
use crate::yul::parser::dialect::llvm::LLVMDialect;
use crate::yul::parser::wrapper::Wrap as _;
use era_compiler_llvm_context::IContext;
create_wrapper!(
    yul_syntax_tools::yul::parser::statement::object::Object<LLVMDialect>,
    WrappedObject
);

impl<D> era_compiler_llvm_context::EraVMWriteLLVM<D> for WrappedObject
where
    D: era_compiler_llvm_context::Dependency,
{
    fn declare(
        &mut self,
        context: &mut era_compiler_llvm_context::EraVMContext<D>,
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
        context: &mut era_compiler_llvm_context::EraVMContext<D>,
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
                WrappedObject(*object).into_llvm(context)?;
            }
            None => {
                let runtime = era_compiler_llvm_context::EraVMRuntime::default();
                runtime.into_llvm(context)?;
            }
        }

        Ok(())
    }
}

impl<D> era_compiler_llvm_context::EVMWriteLLVM<D> for WrappedObject
where
    D: era_compiler_llvm_context::Dependency,
{
    fn declare(
        &mut self,
        _context: &mut era_compiler_llvm_context::EVMContext<D>,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn into_llvm(
        self,
        context: &mut era_compiler_llvm_context::EVMContext<D>,
    ) -> anyhow::Result<()> {
        let mut entry = era_compiler_llvm_context::EVMEntryFunction::new(self.0.code.wrap());
        entry.declare(context)?;
        entry.into_llvm(context)?;
        Ok(())
    }
}
