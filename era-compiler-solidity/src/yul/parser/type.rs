//!
//! The YUL source code type.
//!

use era_yul::yul::parser::r#type::Type as YulType;

use crate::create_wrapper;

create_wrapper!(era_yul::yul::parser::r#type::Type, Type);

///
/// The YUL source code type.
///

impl Type {
    ///
    /// Converts the type into its LLVM.
    ///
    pub fn into_llvm<'ctx, C>(self, context: &C) -> inkwell::types::IntType<'ctx>
    where
        C: era_compiler_llvm_context::IContext<'ctx>,
    {
        match self.0 {
            YulType::Bool => context.integer_type(era_compiler_common::BIT_LENGTH_BOOLEAN),
            YulType::Int(bitlength) => context.integer_type(bitlength),
            YulType::UInt(bitlength) => context.integer_type(bitlength),
            YulType::Custom(_) => context.field_type(),
        }
    }
}
