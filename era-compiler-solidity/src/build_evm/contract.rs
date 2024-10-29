//!
//! The Solidity contract build.
//!

use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use crate::solc::combined_json::contract::Contract as CombinedJsonContract;
use crate::solc::standard_json::output::contract::evm::EVM as StandardJsonOutputContractEVM;
use crate::solc::standard_json::output::contract::Contract as StandardJsonOutputContract;

///
/// The Solidity contract build.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Contract {
    /// The contract name.
    pub name: era_compiler_common::ContractName,
    /// The auxiliary identifier. Used to identify Yul objects.
    pub identifier: String,
    /// The deploy bytecode.
    pub deploy_build: Vec<u8>,
    /// The runtime bytecode.
    pub runtime_build: Vec<u8>,
    /// The metadata hash.
    pub metadata_hash: Option<era_compiler_common::Hash>,
    /// The metadata JSON.
    pub metadata_json: serde_json::Value,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        name: era_compiler_common::ContractName,
        identifier: String,
        deploy_build: Vec<u8>,
        runtime_build: Vec<u8>,
        metadata_hash: Option<era_compiler_common::Hash>,
        metadata_json: serde_json::Value,
    ) -> Self {
        Self {
            name,
            identifier,
            deploy_build,
            runtime_build,
            metadata_hash,
            metadata_json,
        }
    }

    ///
    /// Writes the contract text assembly and bytecode to terminal.
    ///
    /// TODO: output assembly
    ///
    pub fn write_to_terminal(
        self,
        path: String,
        output_metadata: bool,
        _output_assembly: bool,
        output_binary: bool,
    ) -> anyhow::Result<()> {
        writeln!(std::io::stdout(), "\n======= {path} =======")?;
        if output_metadata {
            writeln!(std::io::stdout(), "Metadata:\n{}", self.metadata_json)?;
        }
        if output_binary {
            writeln!(
                std::io::stdout(),
                "Binary:\n{}{}",
                hex::encode(self.deploy_build),
                hex::encode(self.runtime_build),
            )?;
        }

        Ok(())
    }

    ///
    /// Writes the contract text assembly and bytecode to files.
    ///
    /// TODO: output assembly
    ///
    pub fn write_to_directory(
        self,
        output_path: &Path,
        _output_assembly: bool,
        output_binary: bool,
        overwrite: bool,
    ) -> anyhow::Result<()> {
        let file_path = PathBuf::from(self.name.path);
        let file_name = file_path
            .file_name()
            .expect("Always exists")
            .to_str()
            .expect("Always valid");

        let mut output_path = output_path.to_owned();
        output_path.push(file_name);
        std::fs::create_dir_all(output_path.as_path())?;

        if output_binary {
            for (code_type, bytecode) in [
                era_compiler_llvm_context::CodeType::Deploy,
                era_compiler_llvm_context::CodeType::Runtime,
            ]
            .into_iter()
            .zip([self.deploy_build, self.runtime_build].into_iter())
            {
                let output_name = format!(
                    "{}.{code_type}.{}",
                    self.name.name.as_deref().unwrap_or(file_name),
                    era_compiler_common::EXTENSION_EVM_BINARY
                );
                let mut output_path = output_path.clone();
                output_path.push(output_name.as_str());

                if output_path.exists() && !overwrite {
                    anyhow::bail!(
                        "Refusing to overwrite an existing file {output_path:?} (use --overwrite to force)."
                    );
                } else {
                    std::fs::write(
                        output_path.as_path(),
                        hex::encode(bytecode.as_slice()).as_bytes(),
                    )
                    .map_err(|error| anyhow::anyhow!("File {output_path:?} writing: {error}"))?;
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

        let hexadecimal_deploy_bytecode = hex::encode(self.deploy_build);
        let hexadecimal_runtime_bytecode = hex::encode(self.runtime_build);
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
        let deploy_bytecode = hex::encode(self.deploy_build.as_slice());
        let runtime_bytecode = hex::encode(self.runtime_build.as_slice());

        standard_json_contract.metadata = self.metadata_json;
        standard_json_contract
            .evm
            .get_or_insert_with(StandardJsonOutputContractEVM::default)
            .modify_evm(deploy_bytecode, runtime_bytecode);

        Ok(())
    }
}
