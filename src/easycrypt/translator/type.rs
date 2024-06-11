//!
//! Transpilation of YUL types.
//!

use anyhow::Error;

use super::Translator;
use crate::easycrypt::syntax::r#type::Type;
use crate::yul::parser::r#type::Type as YulType;

impl Translator {
    /// Default type to fall back when the type in YUL syntax tree is unknown.
    pub const DEFAULT_TYPE: Type = Type::UInt(256);

    /// Transpile an arbitrary YUL type.
    pub fn transpile_type(_type: &YulType) -> Result<Type, Error> {
        Ok(Self::DEFAULT_TYPE)
        // At this time, all types are represented as U256
        //
        // Ok(match r#type {
        //     YulType::Bool => Type::Bool,
        //     YulType::Int(size) => Type::Int(*size),
        //     YulType::UInt(size) => Type::UInt(*size),
        //     YulType::Custom(custom) => Type::Custom(custom.clone()),
        // })
    }
}
