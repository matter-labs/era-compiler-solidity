//!
//! The Solidity contract build.
//!

use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

use crate::solc::combined_json::contract::Contract as CombinedJsonContract;
use crate::solc::standard_json::output::contract::Contract as StandardJsonOutputContract;

///
/// The Solidity contract build.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Contract {
    /// The contract path.
    pub path: String,
    /// The auxiliary identifier. Used to identify Yul objects.
    pub identifier: String,
    /// The LLVM deploy code module build.
    pub deploy_build: era_compiler_llvm_context::EVMBuild,
    /// The LLVM runtime code module build.
    pub runtime_build: era_compiler_llvm_context::EVMBuild,
    /// The metadata JSON.
    pub metadata_json: serde_json::Value,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        path: String,
        identifier: String,
        deploy_build: era_compiler_llvm_context::EVMBuild,
        runtime_build: era_compiler_llvm_context::EVMBuild,
        metadata_json: serde_json::Value,
    ) -> Self {
        Self {
            path,
            identifier,
            deploy_build,
            runtime_build,
            metadata_json,
        }
    }

    ///
    /// Writes the contract text assembly and bytecode to files.
    ///
    /// TODO: output assembly
    ///
    pub fn write_to_directory(
        self,
        path: &Path,
        _output_assembly: bool,
        output_binary: bool,
        overwrite: bool,
    ) -> anyhow::Result<()> {
        let file_name = Self::short_path(self.path.as_str());

        if output_binary {
            for (code_type, bytecode) in [
                era_compiler_llvm_context::CodeType::Deploy,
                era_compiler_llvm_context::CodeType::Runtime,
            ]
            .into_iter()
            .zip([self.deploy_build.bytecode, self.runtime_build.bytecode].into_iter())
            {
                let file_name = format!(
                    "{}.{}.{}",
                    file_name,
                    code_type,
                    era_compiler_common::EXTENSION_EVM_BINARY
                );
                let mut file_path = path.to_owned();
                file_path.push(file_name);

                if file_path.exists() && !overwrite {
                    eprintln!(
                        "Refusing to overwrite an existing file {file_path:?} (use --overwrite to force)."
                    );
                } else {
                    File::create(&file_path)
                        .map_err(|error| {
                            anyhow::anyhow!("File {:?} creating error: {}", file_path, error)
                        })?
                        .write_all(bytecode.as_slice())
                        .map_err(|error| {
                            anyhow::anyhow!("File {:?} writing error: {}", file_path, error)
                        })?;
                }
            }
        }

        Ok(())
    }

    ///
    /// Writes the contract text assembly and bytecode to the combined JSON.
    ///
    /// TODO: output assembly
    ///
    pub fn write_to_combined_json(
        self,
        combined_json_contract: &mut CombinedJsonContract,
    ) -> anyhow::Result<()> {
        if let Some(metadata) = combined_json_contract.metadata.as_mut() {
            *metadata = self.metadata_json.to_string();
        }

        let hexadecimal_deploy_bytecode = hex::encode(self.deploy_build.bytecode);
        let hexadecimal_runtime_bytecode = hex::encode(self.runtime_build.bytecode);
        match (
            combined_json_contract.bin.as_mut(),
            combined_json_contract.bin_runtime.as_mut(),
        ) {
            (Some(bin), Some(bin_runtime)) => {
                *bin = hexadecimal_deploy_bytecode;
                *bin_runtime = hexadecimal_runtime_bytecode;
            }
            (Some(bin), None) => {
                *bin = hexadecimal_deploy_bytecode;
            }
            (None, Some(bin_runtime)) => {
                *bin_runtime = hexadecimal_runtime_bytecode;
            }
            (None, None) => {}
        }

        Ok(())
    }

    ///
    /// Writes the contract text assembly and bytecode to the standard JSON.
    ///
    /// TODO: output assembly
    ///
    pub fn write_to_standard_json(
        self,
        standard_json_contract: &mut StandardJsonOutputContract,
    ) -> anyhow::Result<()> {
        standard_json_contract.metadata = Some(self.metadata_json);

        let deploy_bytecode = hex::encode(self.deploy_build.bytecode.as_slice());
        let runtime_bytecode = hex::encode(self.runtime_build.bytecode.as_slice());
        if let Some(evm) = standard_json_contract.evm.as_mut() {
            evm.modify_evm(deploy_bytecode, runtime_bytecode);
        }

        Ok(())
    }

    ///
    /// Converts the full path to a short one.
    ///
    pub fn short_path(path: &str) -> &str {
        path.rfind('/')
            .map(|last_slash| &path[last_slash + 1..])
            .unwrap_or_else(|| path)
    }
}
