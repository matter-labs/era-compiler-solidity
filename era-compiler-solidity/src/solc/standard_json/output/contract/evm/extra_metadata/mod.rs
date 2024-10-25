//!
//! The `solc --standard-json` output contract EVM extra metadata.
//!

pub mod recursive_function;

use self::recursive_function::RecursiveFunction;

///
/// The `solc --standard-json` output contract EVM extra metadata.
///
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtraMetadata {
    /// The list of recursive functions.
    #[serde(default)]
    pub recursive_functions: Vec<RecursiveFunction>,
}

impl ExtraMetadata {
    ///
    /// Returns the recursive function reference for the specified tag.
    ///
    pub fn get(
        &self,
        block_key: &era_compiler_llvm_context::BlockKey,
    ) -> Option<&RecursiveFunction> {
        for function in self.recursive_functions.iter() {
            match block_key.code_type {
                era_compiler_llvm_context::CodeType::Deploy => {
                    if function
                        .creation_tag
                        .map(|creation_tag| num::BigUint::from(creation_tag) == block_key.tag)
                        .unwrap_or_default()
                    {
                        return Some(function);
                    }
                }
                era_compiler_llvm_context::CodeType::Runtime => {
                    if function
                        .runtime_tag
                        .map(|runtime_tag| num::BigUint::from(runtime_tag) == block_key.tag)
                        .unwrap_or_default()
                    {
                        return Some(function);
                    }
                }
            }
        }

        None
    }
}
