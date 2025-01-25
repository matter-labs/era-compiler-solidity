//!
//! The Ethereal IR entry function link.
//!

use inkwell::values::BasicValue;

use era_compiler_llvm_context::IContext;

use crate::evmla::ethereal_ir::EtherealIR;

///
/// The Ethereal IR entry function link.
///
/// The link represents branching between deploy and runtime code.
///
#[derive(Debug, Clone)]
pub struct EntryLink {
    /// The code segment.
    pub code_segment: era_compiler_common::CodeSegment,
}

impl EntryLink {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(code_segment: era_compiler_common::CodeSegment) -> Self {
        Self { code_segment }
    }
}

impl era_compiler_llvm_context::EraVMWriteLLVM for EntryLink {
    fn into_llvm(
        self,
        context: &mut era_compiler_llvm_context::EraVMContext,
    ) -> anyhow::Result<()> {
        let target = context
            .get_function(EtherealIR::DEFAULT_ENTRY_FUNCTION_NAME)
            .expect("Always exists")
            .borrow()
            .declaration();
        let is_deploy_code = match self.code_segment {
            era_compiler_common::CodeSegment::Deploy => context
                .integer_type(era_compiler_common::BIT_LENGTH_BOOLEAN)
                .const_int(1, false),
            era_compiler_common::CodeSegment::Runtime => context
                .integer_type(era_compiler_common::BIT_LENGTH_BOOLEAN)
                .const_int(0, false),
        };
        context.build_invoke(
            target,
            &[is_deploy_code.as_basic_value_enum()],
            format!("call_link_{}", EtherealIR::DEFAULT_ENTRY_FUNCTION_NAME).as_str(),
        )?;

        Ok(())
    }
}

impl era_compiler_llvm_context::EVMWriteLLVM for EntryLink {
    fn into_llvm(self, context: &mut era_compiler_llvm_context::EVMContext) -> anyhow::Result<()> {
        let target = context
            .get_function(EtherealIR::DEFAULT_ENTRY_FUNCTION_NAME)
            .expect("Always exists")
            .borrow()
            .declaration();
        context.build_invoke(
            target,
            &[],
            format!("call_link_{}", EtherealIR::DEFAULT_ENTRY_FUNCTION_NAME).as_str(),
        )?;

        Ok(())
    }
}
