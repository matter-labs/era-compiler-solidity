//!
//! Symbol table
//!

use anyhow::Error;

use crate::easycrypt::translator::definition_info::standard_definitions::standard_function_definition;
use crate::easycrypt::translator::definition_info::DefinitionInfo;
use crate::yul::parser::statement::expression::function_call::name::Name as YulName;
use crate::yul::path::full_name::FullName;
use crate::yul::path::symbol_table::SymbolTable;
use crate::yul::path::Path;


pub fn get_definition<'a>(
    environment: &'a SymbolTable<DefinitionInfo>,
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
