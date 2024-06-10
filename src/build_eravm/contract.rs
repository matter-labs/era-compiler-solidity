//!
//! The Solidity contract build.
//!

use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

use crate::solc::combined_json::contract::Contract as CombinedJsonContract;
use crate::solc::standard_json::output::contract::evm::EVM as StandardJsonOutputContractEVM;
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
    pub fn write_to_terminal(
        self,
        path: String,
        output_assembly: bool,
        output_binary: bool,
    ) -> anyhow::Result<()> {
        if output_assembly {
            writeln!(
                std::io::stdout(),
                "Contract `{path}` assembly:\n\n{}",
                self.build.assembly_text
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
        output_assembly: bool,
        output_binary: bool,
        overwrite: bool,
    ) -> anyhow::Result<()> {
        let (file_name, contract_name) = Self::split_path(self.path.as_str());

        if output_assembly {
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
                    .map_err(|error| {
                        anyhow::anyhow!("File {:?} creating error: {}", file_path, error)
                    })?
                    .write_all(self.build.assembly_text.as_bytes())
                    .map_err(|error| {
                        anyhow::anyhow!("File {:?} writing error: {}", file_path, error)
                    })?;
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
                    .map_err(|error| {
                        anyhow::anyhow!("File {:?} creating error: {}", file_path, error)
                    })?
                    .write_all(
                        format!("0x{}", hex::encode(self.build.bytecode.as_slice())).as_bytes(),
                    )
                    .map_err(|error| {
                        anyhow::anyhow!("File {:?} writing error: {}", file_path, error)
                    })?;
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
        if let Some(metadata) = combined_json_contract.metadata.as_mut() {
            *metadata = self.metadata_json.to_string();
        }

        if let Some(asm) = combined_json_contract.asm.as_mut() {
            *asm = serde_json::Value::String(self.build.assembly_text);
        }
        let hexadecimal_bytecode = hex::encode(self.build.bytecode);
        combined_json_contract.bin = Some(hexadecimal_bytecode);
        combined_json_contract
            .bin_runtime
            .clone_from(&combined_json_contract.bin);

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
        standard_json_contract.metadata = Some(self.metadata_json);

        let assembly_text = self.build.assembly_text;
        let bytecode = hex::encode(self.build.bytecode.as_slice());
        match standard_json_contract.evm.as_mut() {
            Some(evm) => evm.modify_eravm(assembly_text, bytecode),
            None => {
                standard_json_contract.evm = Some(StandardJsonOutputContractEVM::new_eravm(
                    assembly_text,
                    bytecode,
                ))
            }
        }

        standard_json_contract.factory_dependencies = Some(self.build.factory_dependencies);
        standard_json_contract.hash = Some(self.build.bytecode_hash);

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
