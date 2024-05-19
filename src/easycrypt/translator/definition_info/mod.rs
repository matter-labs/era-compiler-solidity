//!
//! Static description of a definition in transpiled EasyCrypt code.
//!

pub mod kind;

use crate::easycrypt::syntax::r#type::Type;
use crate::yul::path::full_name::FullName;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DefinitionInfo {
    pub kind: self::kind::Kind,
    pub full_name: FullName,
    pub r#type: Type,
}
