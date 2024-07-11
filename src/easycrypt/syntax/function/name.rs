//!
//! Name of a function in EasyCrypt, which can be a user-defined custom name or
//! one of the pre-defined names such as `lt`.
//!

use std::fmt::Display;

use crate::easycrypt::syntax::Name;
use crate::yul::path::full_name::FullName;

/// Name of a function, which can be a user-defined custom name or one of the pre-defined names such as `lt`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionName {
    /// The user-defined function.
    pub name: Name,
    pub module: Option<Name>,
    // FIXME separation
    pub yul_name: Option<FullName>,
}

impl Display for FunctionName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let FunctionName { name, module, .. } = self;

        if let Some(module) = module {
            f.write_fmt(format_args!("{}.{}", module, name))
        } else {
            f.write_str(name.as_str())
        }
    }
}
