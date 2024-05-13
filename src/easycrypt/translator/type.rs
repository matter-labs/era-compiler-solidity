//!
//! Transpilation of YUL types.
//!

use anyhow::Error;

use crate::easycrypt::syntax::r#type::Type;
use crate::yul::parser::r#type::Type as YulType;

impl crate::Translator {
    /// Default type to fall back when the type in YUL syntax tree is unknown.
    pub const DEFAULT_TYPE: Type = Type::Int(32);

    /// Transpile an arbitrary YUL type.
    pub fn transpile_type(r#type: &YulType) -> Result<Type, Error> {
        Ok(match r#type {
            YulType::Bool => Type::Bool,
            YulType::Int(size) => Type::Int(*size),
            YulType::UInt(size) => Type::UInt(*size),
            YulType::Custom(custom) => Type::Custom(custom.clone()),
        })
    }
}
