//!
//! The Solidity project build.
//!

pub mod contract;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use normpath::PathExt;

use era_solc::CollectableError;

use self::contract::object_format::ObjectFormat;
use self::contract::Contract;

///
/// The Solidity project build.
///
#[derive(Debug)]
pub struct Build {
    /// The contract build data,
    pub results: BTreeMap<String, Result<Contract, era_solc::StandardJsonOutputError>>,
    /// The additional message to output.
    pub messages: Vec<era_solc::StandardJsonOutputError>,
}

impl Build {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        results: BTreeMap<String, Result<Contract, era_solc::StandardJsonOutputError>>,
        messages: &mut Vec<era_solc::StandardJsonOutputError>,
    ) -> Self {
        Self {
            results,
            messages: std::mem::take(messages),
        }
    }

    ///
    /// Links the EraVM build.
    ///
    pub fn link(
        mut self,
        linker_symbols: BTreeMap<String, [u8; era_compiler_common::BYTE_LENGTH_ETH_ADDRESS]>,
    ) -> anyhow::Result<Self> {
        let mut contracts: HashMap<String, Contract> = self
            .results
            .into_iter()
            .map(|(path, result)| (path, result.expect("Cannot link a project with errors")))
            .collect();

        loop {
            let mut linkage_data = BTreeMap::new();
            let unlinked_satisfied_contracts: BTreeMap<&String, &Contract> = contracts
                .iter()
                .filter(|(_path, contract)| {
                    contract.object_format == ObjectFormat::ELF
                        && contract.factory_dependencies.iter().all(|dependency| {
                            contracts
                                .get(dependency)
                                .expect("Always exists")
                                .object_format
                                == ObjectFormat::Raw
                        })
                        && contract.factory_dependencies.iter().all(|dependency| {
                            contracts
                                .get(dependency)
                                .expect("Always exists")
                                .object_format
                                == ObjectFormat::Raw
                        })
                })
                .collect();
            if unlinked_satisfied_contracts.is_empty() {
                break;
            }

            for (path, contract) in unlinked_satisfied_contracts.into_iter() {
                let factory_dependencies: BTreeMap<
                    String,
                    [u8; era_compiler_common::BYTE_LENGTH_FIELD],
                > = contract
                    .factory_dependencies
                    .iter()
                    .map(|dependency| {
                        let bytecode_hash = contracts
                            .get(dependency)
                            .expect("Always exists")
                            .build
                            .bytecode_hash
                            .to_owned()
                            .expect("Always exists");
                        (dependency.to_owned(), bytecode_hash)
                    })
                    .collect();

                let memory_buffer = inkwell::memory_buffer::MemoryBuffer::create_from_memory_range(
                    contract.build.bytecode.as_slice(),
                    path.as_str(),
                    false,
                );
                match era_compiler_llvm_context::eravm_link(
                    memory_buffer,
                    &linker_symbols,
                    &factory_dependencies,
                ) {
                    Ok((memory_buffer_linked, bytecode_hash)) => {
                        linkage_data.insert(path.to_owned(), (memory_buffer_linked, bytecode_hash));
                    }
                    Err(error) => self
                        .messages
                        .push(era_solc::StandardJsonOutputError::new_error(
                            error, None, None,
                        )),
                }
            }

            let mut linked_contracts = 0;
            for (path, (memory_buffer_linked, bytecode_hash)) in linkage_data.into_iter() {
                let contract = contracts.get(path.as_str()).expect("Always exists");
                let factory_dependencies_resolved = contract
                    .factory_dependencies
                    .iter()
                    .map(|dependency| {
                        (
                            contracts
                                .get(dependency)
                                .expect("Always exists")
                                .build
                                .bytecode_hash
                                .to_owned()
                                .expect("Always exists"),
                            dependency.to_owned(),
                        )
                    })
                    .collect();

                let contract = contracts.get_mut(path.as_str()).expect("Always exists");
                contract.build.bytecode = memory_buffer_linked.as_slice().to_vec();
                contract.build.bytecode_hash = bytecode_hash;
                contract.factory_dependencies_resolved = factory_dependencies_resolved;
                contract.object_format = if memory_buffer_linked.is_elf_eravm() {
                    ObjectFormat::ELF
                } else {
                    if let ObjectFormat::ELF = contract.object_format {
                        linked_contracts += 1;
                    }
                    ObjectFormat::Raw
                };
            }
            if linked_contracts == 0 {
                break;
            }
        }

        Ok(Self::new(
            contracts
                .into_iter()
                .map(|(path, contract)| (path, Ok(contract)))
                .collect(),
            &mut self.messages,
        ))
    }

    ///
    /// Writes all contracts to the terminal.
    ///
    pub fn write_to_terminal(
        mut self,
        output_metadata: bool,
        output_assembly: bool,
        output_binary: bool,
    ) -> anyhow::Result<()> {
        self.take_and_write_warnings();
        self.exit_on_error();

        if !output_metadata && !output_assembly && !output_binary {
            writeln!(
                std::io::stderr(),
                "Compiler run successful. No output requested. Use flags --metadata, --asm, --bin."
            )?;
            return Ok(());
        }

        for (path, build) in self.results.into_iter() {
            build
                .expect("Always valid")
                .write_to_terminal(path, output_metadata, output_binary)?;
        }

        Ok(())
    }

    ///
    /// Writes all contracts to the specified directory.
    ///
    pub fn write_to_directory(
        mut self,
        output_directory: &Path,
        output_metadata: bool,
        output_binary: bool,
        overwrite: bool,
    ) -> anyhow::Result<()> {
        self.take_and_write_warnings();
        self.exit_on_error();

        std::fs::create_dir_all(output_directory)?;

        for build in self.results.into_values() {
            build.expect("Always valid").write_to_directory(
                output_directory,
                output_metadata,
                output_binary,
                overwrite,
            )?;
        }

        writeln!(
            std::io::stderr(),
            "Compiler run successful. Artifact(s) can be found in directory {output_directory:?}."
        )?;
        Ok(())
    }

    ///
    /// Writes all contracts assembly and bytecode to the standard JSON.
    ///
    pub fn write_to_standard_json(
        self,
        standard_json: &mut era_solc::StandardJsonOutput,
        solc_version: Option<&era_solc::Version>,
    ) -> anyhow::Result<()> {
        let mut errors = Vec::with_capacity(self.results.len());
        for (full_path, build) in self.results.into_iter() {
            let mut full_path_split = full_path.split(':');
            let path = full_path_split.next().expect("Always exists");
            let name = full_path_split.next().unwrap_or(path);

            match build {
                Ok(build) => match standard_json
                    .contracts
                    .get_mut(path)
                    .and_then(|contracts| contracts.get_mut(name))
                {
                    Some(contract) => {
                        build.write_to_standard_json(contract)?;
                    }
                    None => {
                        let contracts = standard_json.contracts.entry(path.to_owned()).or_default();
                        let mut contract = era_solc::StandardJsonOutputContract::default();
                        build.write_to_standard_json(&mut contract)?;
                        contracts.insert(name.to_owned(), contract);
                    }
                },
                Err(error) => errors.push(error),
            }
        }

        standard_json.errors.extend(errors);
        if let Some(solc_version) = solc_version {
            standard_json.version = Some(solc_version.default.to_string());
            standard_json.long_version = Some(solc_version.long.to_owned());
        }

        Ok(())
    }

    ///
    /// Writes all contracts assembly and bytecode to the combined JSON.
    ///
    pub fn write_to_combined_json(
        mut self,
        combined_json: &mut era_solc::CombinedJson,
    ) -> anyhow::Result<()> {
        self.take_and_write_warnings();
        self.exit_on_error();

        for (path, build) in self.results.into_iter() {
            let combined_json_contract = combined_json
                .contracts
                .iter_mut()
                .find_map(|(json_path, contract)| {
                    let path = PathBuf::from(&path[..path.rfind(':').expect("Always exists")])
                        .normalize()
                        .expect("Path normalization error");
                    let json_path =
                        PathBuf::from(&json_path[..json_path.rfind(':').expect("Always exists")])
                            .normalize()
                            .expect("Path normalization error");

                    if path.ends_with(json_path) {
                        Some(contract)
                    } else {
                        None
                    }
                })
                .ok_or_else(|| anyhow::anyhow!("contract `{path}` not found in the project"))?;

            build
                .expect("Always valid")
                .write_to_combined_json(combined_json_contract)?;
        }

        Ok(())
    }
}

impl era_solc::CollectableError for Build {
    fn errors(&self) -> Vec<&era_solc::StandardJsonOutputError> {
        self.results
            .values()
            .filter_map(|build| build.as_ref().err())
            .collect()
    }

    fn warnings(&self) -> Vec<&era_solc::StandardJsonOutputError> {
        self.messages.iter().collect()
    }

    fn remove_warnings(&mut self) {
        self.messages.clear();
    }
}
