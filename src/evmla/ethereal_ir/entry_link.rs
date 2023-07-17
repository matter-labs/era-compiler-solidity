//!
//! The Ethereal IR entry function link.
//!

use inkwell::values::BasicValue;

use crate::evmla::ethereal_ir::EtherealIR;

///
/// The Ethereal IR entry function link.
///
/// The link represents branching between the deploy and runtime code.
///
#[derive(Debug, Clone)]
pub struct EntryLink {
    /// The code part type.
    pub code_type: compiler_llvm_context::CodeType,
}

impl EntryLink {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(code_type: compiler_llvm_context::CodeType) -> Self {
        Self { code_type }
    }
}

impl<D> compiler_llvm_context::WriteLLVM<D> for EntryLink
where
    D: compiler_llvm_context::Dependency + Clone,
{
    fn into_llvm(self, context: &mut compiler_llvm_context::Context<D>) -> anyhow::Result<()> {
        let target = context
            .get_function(EtherealIR::DEFAULT_ENTRY_FUNCTION_NAME)
            .expect("Always exists")
            .borrow()
            .declaration();
        let is_deploy_code = match self.code_type {
            compiler_llvm_context::CodeType::Deploy => context
                .integer_type(compiler_common::BIT_LENGTH_BOOLEAN)
                .const_int(1, false),
            compiler_llvm_context::CodeType::Runtime => context
                .integer_type(compiler_common::BIT_LENGTH_BOOLEAN)
                .const_int(0, false),
        };
        context.build_invoke(
            target,
            &[is_deploy_code.as_basic_value_enum()],
            format!("call_link_{}", EtherealIR::DEFAULT_ENTRY_FUNCTION_NAME).as_str(),
        );

        Ok(())
    }
}
