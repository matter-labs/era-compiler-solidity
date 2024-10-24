//!
//! The Solidity contract build.
//!

use std::collections::HashSet;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use crate::solc::combined_json::contract::Contract as CombinedJsonContract;
use crate::solc::standard_json::output::contract::eravm::EraVM as StandardJsonOutputContractEraVM;
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
        name: era_compiler_common::ContractName,
        identifier: String,
        build: era_compiler_llvm_context::EraVMBuild,
        metadata_json: serde_json::Value,
        factory_dependencies: HashSet<String>,
    ) -> Self {
        Self {
            name,
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
        output_metadata: bool,
        output_binary: bool,
    ) -> anyhow::Result<()> {
        writeln!(std::io::stdout(), "\n======= {path} =======",)?;
        if let Some(assembly) = self.build.assembly {
            writeln!(std::io::stdout(), "EraVM assembly:\n{assembly}")?;
        }
        if output_metadata {
            writeln!(std::io::stdout(), "Metadata:\n{}", self.metadata_json)?;
        }
        if output_binary {
            writeln!(
                std::io::stdout(),
                "Binary:\n{}",
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
        output_path: &Path,
        output_metadata: bool,
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

        if output_metadata {
            let output_name = format!(
                "{}_meta.{}",
                self.name.name.as_deref().unwrap_or(file_name),
                era_compiler_common::EXTENSION_JSON,
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
                    self.metadata_json.to_string().as_bytes(),
                )
                .map_err(|error| anyhow::anyhow!("File {output_path:?} writing: {error}"))?;
            }
        }

        if let Some(assembly) = self.build.assembly {
            let output_name = format!(
                "{}.{}",
                self.name.name.as_deref().unwrap_or(file_name),
                era_compiler_common::EXTENSION_ERAVM_ASSEMBLY
            );
            let mut output_path = output_path.clone();
            output_path.push(output_name.as_str());

            if output_path.exists() && !overwrite {
                anyhow::bail!(
                    "Refusing to overwrite an existing file {output_path:?} (use --overwrite to force)."
                );
            } else {
                std::fs::write(output_path.as_path(), assembly.as_bytes())
                    .map_err(|error| anyhow::anyhow!("File {output_path:?} writing: {error}"))?;
            }
        }

        if output_binary {
            let output_name = format!(
                "{}.{}",
                self.name.name.as_deref().unwrap_or(file_name),
                era_compiler_common::EXTENSION_ERAVM_BINARY
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
                    hex::encode(self.build.bytecode.as_slice()).as_bytes(),
                )
                .map_err(|error| anyhow::anyhow!("File {output_path:?} writing: {error}"))?;
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
        standard_json_contract.eravm = Some(StandardJsonOutputContractEraVM::new(
            bytecode.clone(),
            assembly.clone(),
        ));
        standard_json_contract
            .evm
            .get_or_insert_with(StandardJsonOutputContractEVM::default)
            .modify_eravm(bytecode, assembly);
        standard_json_contract
            .factory_dependencies
            .extend(self.build.factory_dependencies);
        standard_json_contract.hash = self.build.bytecode_hash.map(hex::encode);

        Ok(())
    }
}
