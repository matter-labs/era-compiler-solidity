//!
//! The YUL source code type.
//!

use yul_syntax_tools::yul::parser::r#type::Type;

use crate::create_wrapper;

create_wrapper!(yul_syntax_tools::yul::parser::r#type::Type, WrappedType);

///
/// The YUL source code type.
///

impl WrappedType {
    ///
    /// Converts the type into its LLVM.
    ///
    pub fn into_llvm<'ctx, C>(self, context: &C) -> inkwell::types::IntType<'ctx>
    where
        C: era_compiler_llvm_context::IContext<'ctx>,
    {
        match self.0 {
            Type::Bool => context.integer_type(era_compiler_common::BIT_LENGTH_BOOLEAN),
            Type::Int(bitlength) => context.integer_type(bitlength),
            Type::UInt(bitlength) => context.integer_type(bitlength),
            Type::Custom(_) => context.field_type(),
        }
    }
}
