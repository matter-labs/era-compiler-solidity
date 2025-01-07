//!
//! The Solidity contract build.
//!

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

///
/// The Solidity contract build.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Contract {
    /// The contract name.
    pub name: era_compiler_common::ContractName,
    /// The LLVM module build.
    pub build: era_compiler_llvm_context::EraVMBuild,
    /// The metadata JSON.
    pub metadata_json: serde_json::Value,
    /// The unlinked missing libraries.
    pub missing_libraries: BTreeSet<String>,
    /// The unresolved factory dependencies.
    pub factory_dependencies: BTreeSet<String>,
    /// The resolved factory dependencies.
    pub factory_dependencies_resolved:
        HashMap<[u8; era_compiler_common::BYTE_LENGTH_FIELD], String>,
    /// The binary object format.
    pub object_format: era_compiler_common::ObjectFormat,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        name: era_compiler_common::ContractName,
        build: era_compiler_llvm_context::EraVMBuild,
        metadata_json: serde_json::Value,
        missing_libraries: BTreeSet<String>,
        factory_dependencies: BTreeSet<String>,
        object_format: era_compiler_common::ObjectFormat,
    ) -> Self {
        Self {
            name,
            build,
            metadata_json,
            missing_libraries,
            factory_dependencies,
            factory_dependencies_resolved: HashMap::new(),
            object_format,
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
            writeln!(std::io::stdout(), "Assembly:\n{assembly}")?;
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
    /// Writes the contract text assembly and bytecode to the standard JSON.
    ///
    pub fn write_to_standard_json(
        self,
        standard_json_contract: &mut era_solc::StandardJsonOutputContract,
    ) -> anyhow::Result<()> {
        let bytecode = hex::encode(self.build.bytecode.as_slice());
        let assembly = self.build.assembly;

        standard_json_contract.metadata = self.metadata_json;
        standard_json_contract.eravm = Some(era_solc::StandardJsonOutputContractEraVM::new(
            bytecode.clone(),
            assembly.clone(),
        ));
        standard_json_contract
            .evm
            .get_or_insert_with(era_solc::StandardJsonOutputContractEVM::default)
            .modify_eravm(bytecode, assembly);
        standard_json_contract.hash = self.build.bytecode_hash.map(hex::encode);
        standard_json_contract
            .missing_libraries
            .extend(self.missing_libraries);
        standard_json_contract
            .factory_dependencies_unlinked
            .extend(self.factory_dependencies);
        standard_json_contract.factory_dependencies.extend(
            self.factory_dependencies_resolved
                .into_iter()
                .map(|(hash, path)| (hex::encode(hash), path)),
        );
        standard_json_contract.object_format = Some(self.object_format);

        Ok(())
    }

    ///
    /// Writes the contract text assembly and bytecode to the combined JSON.
    ///
    pub fn write_to_combined_json(
        self,
        combined_json_contract: &mut era_solc::CombinedJsonContract,
    ) -> anyhow::Result<()> {
        let hexadecimal_bytecode = hex::encode(self.build.bytecode);

        if let Some(metadata) = combined_json_contract.metadata.as_mut() {
            *metadata = self.metadata_json.to_string();
        }

        combined_json_contract.assembly = self.build.assembly;
        combined_json_contract.bin = Some(hexadecimal_bytecode);
        combined_json_contract
            .bin_runtime
            .clone_from(&combined_json_contract.bin);

        combined_json_contract
            .missing_libraries
            .extend(self.missing_libraries);
        combined_json_contract
            .factory_deps_unlinked
            .extend(self.factory_dependencies);
        combined_json_contract.factory_deps.extend(
            self.factory_dependencies_resolved
                .into_iter()
                .map(|(hash, path)| (hex::encode(hash), path)),
        );
        combined_json_contract.object_format = Some(self.object_format);

        Ok(())
    }
}
