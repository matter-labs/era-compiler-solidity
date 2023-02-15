//!
//! The Solidity contract build.
//!

use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::solc::combined_json::contract::Contract as CombinedJsonContract;
use crate::solc::standard_json::output::contract::evm::EVM as StandardJsonOutputContractEVM;
use crate::solc::standard_json::output::contract::Contract as StandardJsonOutputContract;

///
/// The Solidity contract build.
///
#[derive(Debug)]
pub struct Contract {
    /// The contract path.
    pub path: String,
    /// The auxiliary identifier. Used to identify Yul objects.
    pub identifier: String,
    /// The LLVM module build.
    pub build: compiler_llvm_context::Build,
    /// The ABI specification.
    pub abi: Option<serde_json::Value>,
    /// The method identifiers.
    pub method_identifiers: Option<BTreeMap<String, String>>,
    /// The storage layout.
    pub storage_layout: Option<serde_json::Value>,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        path: String,
        identifier: String,
        build: compiler_llvm_context::Build,
        abi: Option<serde_json::Value>,
        method_identifiers: Option<BTreeMap<String, String>>,
        storage_layout: Option<serde_json::Value>,
    ) -> Self {
        Self {
            path,
            identifier,
            build,
            abi,
            method_identifiers,
            storage_layout,
        }
    }

    ///
    /// Writes the contract text assembly and bytecode to files.
    ///
    pub fn write_to_directory(
        self,
        path: &Path,
        output_assembly: bool,
        output_binary: bool,
        output_abi: bool,
        overwrite: bool,
    ) -> anyhow::Result<()> {
        let file_name = Self::short_path(self.path.as_str());

        if output_assembly {
            let file_name = format!(
                "{}.{}",
                file_name,
                compiler_common::EXTENSION_ZKEVM_ASSEMBLY
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
                    .write_all(self.build.assembly_text.as_bytes())
                    .map_err(|error| {
                        anyhow::anyhow!("File {:?} writing error: {}", file_path, error)
                    })?;
            }
        }

        if output_binary {
            let file_name = format!("{}.{}", file_name, compiler_common::EXTENSION_ZKEVM_BINARY);
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
                    .write_all(self.build.bytecode.as_slice())
                    .map_err(|error| {
                        anyhow::anyhow!("File {:?} writing error: {}", file_path, error)
                    })?;
            }
        }

        if let Some(abi) = self.abi {
            if output_abi {
                let file_name = format!("{}.{}", file_name, compiler_common::EXTENSION_ABI);
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
                        .write_all(abi.to_string().as_bytes())
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
    pub fn write_to_combined_json(
        self,
        combined_json_contract: &mut CombinedJsonContract,
    ) -> anyhow::Result<()> {
        combined_json_contract.hashes = self.method_identifiers;
        combined_json_contract.abi = self.abi;

        if let Some(asm) = combined_json_contract.asm.as_mut() {
            *asm = serde_json::Value::String(self.build.assembly_text);
        }

        let hexadecimal_bytecode = hex::encode(self.build.bytecode);
        match (
            combined_json_contract.bin.as_mut(),
            combined_json_contract.bin_runtime.as_mut(),
        ) {
            (Some(bin), Some(bin_runtime)) => {
                *bin = hexadecimal_bytecode;
                *bin_runtime = bin.clone();
            }
            (Some(bin), None) => {
                *bin = hexadecimal_bytecode;
            }
            (None, Some(bin_runtime)) => {
                *bin_runtime = hexadecimal_bytecode;
            }
            (None, None) => {}
        }

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
        standard_json_contract.ir_optimized = None;

        standard_json_contract.abi = self.abi;
        standard_json_contract.storage_layout = self.storage_layout;

        let assembly_text = self.build.assembly_text;
        let bytecode = hex::encode(self.build.bytecode.as_slice());
        standard_json_contract.evm = Some(StandardJsonOutputContractEVM::new(
            assembly_text,
            bytecode,
            self.method_identifiers,
        ));

        standard_json_contract.factory_dependencies = Some(self.build.factory_dependencies);
        standard_json_contract.hash = Some(self.build.hash);

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
