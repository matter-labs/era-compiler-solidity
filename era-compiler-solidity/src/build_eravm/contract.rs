//!
//! The Solidity contract build.
//!

use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::solc::combined_json::contract::Contract as CombinedJsonContract;
use crate::solc::standard_json::output::contract::evm::EVM as StandardJsonOutputContractEVM;
use crate::solc::standard_json::output::contract::Contract as StandardJsonOutputContract;

///
/// The Solidity contract build.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Contract {
    /// The contract path.
    pub path: String,
    /// The auxiliary identifier. Used to identify Yul objects.
    pub identifier: String,
    /// The LLVM module build.
    pub build: era_compiler_llvm_context::EraVMBuild,
    /// The metadata JSON.
    pub metadata_json: serde_json::Value,
    /// The factory dependencies.
    pub factory_dependencies: HashSet<String>,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        path: String,
        identifier: String,
        build: era_compiler_llvm_context::EraVMBuild,
        metadata_json: serde_json::Value,
        factory_dependencies: HashSet<String>,
    ) -> Self {
        Self {
            path,
            identifier,
            build,
            metadata_json,
            factory_dependencies,
        }
    }

    ///
    /// Writes the contract text assembly and bytecode to terminal.
    ///
    pub fn write_to_terminal(self, path: String, output_binary: bool) -> anyhow::Result<()> {
        if let Some(assembly) = self.build.assembly {
            writeln!(
                std::io::stdout(),
                "Contract `{path}` assembly:\n\n{assembly}",
            )?;
        }
        if output_binary {
            writeln!(
                std::io::stdout(),
                "Contract `{path}` bytecode: 0x{}",
                hex::encode(self.build.bytecode)
            )?;
        }

        Ok(())
    }

    ///
    /// Writes the contract text assembly and bytecode to files.
    ///
    pub fn write_to_directory(
        self,
        path: &Path,
        output_binary: bool,
        overwrite: bool,
    ) -> anyhow::Result<()> {
        let (file_name, contract_name) = Self::split_path(self.path.as_str());

        if let Some(assembly) = self.build.assembly {
            let output_name = format!(
                "{}.{}",
                contract_name,
                era_compiler_common::EXTENSION_ERAVM_ASSEMBLY
            );
            let mut file_path = path.to_owned();
            file_path.push(file_name.as_str());
            std::fs::create_dir_all(file_path.as_path())?;
            file_path.push(output_name.as_str());

            if file_path.exists() && !overwrite {
                anyhow::bail!(
                    "Refusing to overwrite an existing file {file_path:?} (use --overwrite to force)."
                );
            } else {
                File::create(&file_path)
                    .map_err(|error| anyhow::anyhow!("File {:?} creating: {}", file_path, error))?
                    .write_all(assembly.as_bytes())
                    .map_err(|error| anyhow::anyhow!("File {:?} writing: {}", file_path, error))?;
            }
        }

        if output_binary {
            let output_name = format!(
                "{}.{}",
                contract_name,
                era_compiler_common::EXTENSION_ERAVM_BINARY
            );
            let mut file_path = path.to_owned();
            file_path.push(file_name.as_str());
            std::fs::create_dir_all(file_path.as_path())?;
            file_path.push(output_name.as_str());

            if file_path.exists() && !overwrite {
                anyhow::bail!(
                    "Refusing to overwrite an existing file {file_path:?} (use --overwrite to force)."
                );
            } else {
                File::create(&file_path)
                    .map_err(|error| anyhow::anyhow!("File {:?} creating: {}", file_path, error))?
                    .write_all(
                        format!("0x{}", hex::encode(self.build.bytecode.as_slice())).as_bytes(),
                    )
                    .map_err(|error| anyhow::anyhow!("File {:?} writing: {}", file_path, error))?;
            }
        }

        Ok(())
    }

    ///
    /// Writes the contract text assembly and bytecode to the combined JSON.
    ///
    pub fn write_to_combined_json(
        self,
        combined_json_contract: &mut CombinedJsonContract,
    ) -> anyhow::Result<()> {
        let hexadecimal_bytecode = hex::encode(self.build.bytecode);

        if let Some(metadata) = combined_json_contract.metadata.as_mut() {
            *metadata = self.metadata_json.to_string();
        }
        combined_json_contract.bin = Some(hexadecimal_bytecode);
        combined_json_contract
            .bin_runtime
            .clone_from(&combined_json_contract.bin);

        combined_json_contract.assembly = self.build.assembly.map(serde_json::Value::String);
        combined_json_contract.factory_deps = Some(self.build.factory_dependencies);

        Ok(())
    }

    ///
    /// Writes the contract text assembly and bytecode to the standard JSON.
    ///
    pub fn write_to_standard_json(
        self,
        standard_json_contract: &mut StandardJsonOutputContract,
    ) -> anyhow::Result<()> {
        let bytecode = hex::encode(self.build.bytecode.as_slice());
        let assembly = self.build.assembly;

        standard_json_contract.metadata = Some(self.metadata_json);
        standard_json_contract
            .evm
            .get_or_insert_with(StandardJsonOutputContractEVM::default)
            .modify_eravm(bytecode, assembly);
        standard_json_contract.factory_dependencies = Some(self.build.factory_dependencies);
        standard_json_contract.hash = Some(hex::encode(self.build.bytecode_hash));

        Ok(())
    }

    ///
    /// Extracts the file and contract names from the full path.
    ///
    pub fn split_path(path: &str) -> (String, String) {
        let path = path.trim().replace(['\\', ':'], "/");
        let mut path_iterator = path.split('/').rev();
        let contract_name = path_iterator.next().expect("Always exists");
        let file_name = path_iterator.next().expect("Always exists");
        (file_name.to_owned(), contract_name.to_owned())
    }
}
