//!
//! Static description of a definition (standard or non-standard) in transpiled
//! EasyCrypt code. Common part for all definitions.
//!

pub mod attributes;
pub mod kind;
pub mod standard_definitions;
pub mod usage;

use anyhow::Error;

use crate::easycrypt::syntax::r#type::Type;
use crate::easycrypt::translator::definition_info::standard_definitions::standard_function_definition;
use crate::yul::parser::statement::expression::function_call::name::Name as YulName;
use crate::yul::path::full_name::FullName;
use crate::yul::path::symbol_table::SymbolTable;
use crate::yul::path::Path;

use self::kind::Kind;

/// Static description of a definition (standard or non-standard) in transpiled
/// EasyCrypt code. Common part for all definitions.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DefinitionInfo {
    /// Kind of definition: binop/unop, function, procedure etc.
    pub kind: Kind,
    /// Full path from the root of YUL syntax tree.
    pub full_name: FullName,
    /// Type of definition.
    pub r#type: Type,
}
// FIXME inefficient
pub fn get_definition(
    environment: &SymbolTable<DefinitionInfo>,
    name: &YulName,
    path: &Path,
) -> Result<DefinitionInfo, Error> {
    match name {
        YulName::UserDefined(name_str) => {
            let full_name = FullName {
                name: name_str.to_string(),
                path: path.clone(),
            };
            let definition = environment.get(&full_name);
            match definition {
                Some(def) => Ok(def.clone()),
                None => anyhow::bail!(
                    "Can not find user-defined function {} among the definitions",
                    name_str
                ),
            }
        }
        standard_function => standard_function_definition(standard_function),
    }
}
