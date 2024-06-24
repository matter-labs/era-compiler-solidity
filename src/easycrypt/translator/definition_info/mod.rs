//!
//! Static description of a definition (standard or non-standard) in transpiled
//! EasyCrypt code. Common part for all definitions.
//!

pub mod attributes;
pub mod kind;
pub mod usage;

use anyhow::Error;

use crate::easycrypt::syntax::r#type::Type;
use crate::yul::parser::statement::expression::function_call::name::Name as YulName;
use crate::yul::path::full_name::FullName;
use crate::yul::path::symbol_table::SymbolTable;
use crate::yul::path::Path;
use crate::yul::printer::name_identifier;

use self::kind::Kind;

/// Static description of a definition (standard or non-standard) in transpiled
/// EasyCrypt code. Common part for all definitions.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DefinitionInfo {
    /// Kind of definition: binop/unop, function, procedure etc.
    pub kind: Kind,
    /// Full path from the root of YUL syntax tree.
    pub yul_name: FullName,
    /// Type of definition.
    pub r#type: Type,
}

pub fn get_definition(
    environment: &SymbolTable<DefinitionInfo>,
    name: &YulName,
    path: &Path,
) -> Result<DefinitionInfo, Error> {
    let string_representation = name_identifier(name);

    let full_name = FullName {
        name: string_representation.clone(),
        path: path.clone(),
    };
    let definition = environment.get(&full_name);
    match definition {
        Some(def) => Ok(def.clone()),
        None => anyhow::bail!("Missing definition: \"{}\"", &string_representation),
    }
}
