//!
//! The Solidity project build.
//!

pub mod contract;

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use normpath::PathExt;

use era_solc::CollectableError;

use self::contract::object::Object as ContractObject;
use self::contract::Contract;

///
/// The Solidity project build.
///
#[derive(Debug)]
pub struct Build {
    /// The contract data,
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
    /// Links the EVM build.
    ///
    pub fn link(
        mut self,
        linker_symbols: BTreeMap<String, [u8; era_compiler_common::BYTE_LENGTH_ETH_ADDRESS]>,
        cbor_data: Option<Vec<(String, semver::Version)>>,
    ) -> Self {
        let mut contracts: BTreeMap<String, Contract> = self
            .results
            .into_iter()
            .map(|(path, result)| (path, result.expect("Cannot link a project with errors")))
            .collect();

        loop {
            let assembled_objects_data = {
                let all_objects = contracts
                    .iter()
                    .flat_map(|(_path, contract)| {
                        vec![&contract.deploy_object, &contract.runtime_object]
                    })
                    .collect::<Vec<&ContractObject>>();
                let assembleable_objects = all_objects
                    .iter()
                    .filter(|object| {
                        !object.is_assembled
                            && object.format == era_compiler_common::ObjectFormat::ELF
                            && object.dependencies.inner.iter().all(|dependency| {
                                all_objects
                                    .iter()
                                    .find(|object| {
                                        object.identifier.as_str() == dependency.as_str()
                                    })
                                    .map(|object| object.is_assembled)
                                    .unwrap_or_default()
                            })
                    })
                    .copied()
                    .collect::<Vec<_>>();
                if assembleable_objects.is_empty() {
                    break;
                }

                let mut assembled_objects_data = Vec::with_capacity(assembleable_objects.len());
                for object in assembleable_objects.into_iter() {
                    let assembled_object =
                        match object.assemble(all_objects.as_slice(), cbor_data.clone()) {
                            Ok(assembled_object) => assembled_object,
                            Err(error) => {
                                self.messages
                                    .push(era_solc::StandardJsonOutputError::new_error(
                                        &error, None, None,
                                    ));
                                return Self::new(BTreeMap::new(), &mut self.messages);
                            }
                        };
                    assembled_objects_data.push((
                        object.contract_name.full_path.to_owned(),
                        object.code_segment,
                        assembled_object,
                    ));
                }
                assembled_objects_data
            };

            for (full_path, code_segment, assembled_object) in assembled_objects_data.into_iter() {
                let contract = contracts
                    .get_mut(full_path.as_str())
                    .expect("Always exists");
                let object = match code_segment {
                    era_compiler_common::CodeSegment::Deploy => &mut contract.deploy_object,
                    era_compiler_common::CodeSegment::Runtime => &mut contract.runtime_object,
                };
                object.bytecode = assembled_object.as_slice().to_owned();
                object.is_assembled = true;
            }
        }

        for contract in contracts.values_mut() {
            for object in [&mut contract.deploy_object, &mut contract.runtime_object].into_iter() {
                if let Err(error) = object.link(&linker_symbols) {
                    self.messages
                        .push(era_solc::StandardJsonOutputError::new_error(
                            &error, None, None,
                        ));
                    return Self::new(BTreeMap::new(), &mut self.messages);
                }
            }
        }

        Self::new(
            contracts
                .into_iter()
                .map(|(path, contract)| (path, Ok(contract)))
                .collect(),
            &mut self.messages,
        )
    }

    ///
    /// Writes all contracts to the terminal.
    ///
    pub fn write_to_terminal(
        mut self,
        output_metadata: bool,
        output_binary: bool,
    ) -> anyhow::Result<()> {
        self.take_and_write_warnings();
        self.exit_on_error();

        if !output_metadata && !output_binary {
            writeln!(
                std::io::stderr(),
                "Compiler run successful. No output requested. Use flags --metadata, --bin."
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
        for result in self.results.into_values() {
            let build = match result {
                Ok(build) => {
                    errors.extend(
                        build
                            .deploy_object
                            .warnings
                            .iter()
                            .chain(build.runtime_object.warnings.iter())
                            .map(|error| {
                                era_solc::StandardJsonOutputError::new_warning(
                                    error.to_string(),
                                    Some(era_solc::StandardJsonOutputErrorSourceLocation::new(
                                        build.name.full_path.clone(),
                                    )),
                                    None,
                                )
                            })
                            .collect::<Vec<era_solc::StandardJsonOutputError>>(),
                    );
                    build
                }
                Err(error) => {
                    errors.push(error);
                    continue;
                }
            };
            let name = build.name.clone();

            match standard_json
                .contracts
                .get_mut(name.path.as_str())
                .and_then(|contracts| {
                    contracts.get_mut(name.name.as_deref().unwrap_or(name.path.as_str()))
                }) {
                Some(contract) => {
                    build.write_to_standard_json(contract)?;
                }
                None => {
                    let contracts = standard_json
                        .contracts
                        .entry(name.path.clone())
                        .or_default();
                    let mut contract = era_solc::StandardJsonOutputContract::default();
                    build.write_to_standard_json(&mut contract)?;
                    contracts.insert(name.name.unwrap_or(name.path), contract);
                }
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

        for result in self.results.into_values() {
            let build = result.expect("Exits on an error above");
            let name = build.name.clone();

            let combined_json_contract =
                match combined_json
                    .contracts
                    .iter_mut()
                    .find_map(|(json_path, contract)| {
                        if Self::normalize_full_path(name.full_path.as_str())
                            .ends_with(Self::normalize_full_path(json_path).as_str())
                        {
                            Some(contract)
                        } else {
                            None
                        }
                    }) {
                    Some(contract) => contract,
                    None => {
                        combined_json.contracts.insert(
                            name.full_path.clone(),
                            era_solc::CombinedJsonContract::default(),
                        );
                        combined_json
                            .contracts
                            .get_mut(name.full_path.as_str())
                            .expect("Always exists")
                    }
                };

            build.write_to_combined_json(combined_json_contract)?;
        }

        Ok(())
    }

    ///
    /// Normalizes full contract path.
    ///
    /// # Panics
    /// If the path does not contain a colon.
    ///
    fn normalize_full_path(path: &str) -> String {
        let mut iterator = path.split(':');
        let path = iterator.next().expect("Always exists");
        let name = iterator.next().expect("Always exists");

        let mut full_path = PathBuf::from(path)
            .normalize()
            .expect("Path normalization error")
            .as_os_str()
            .to_string_lossy()
            .into_owned();
        full_path.push(':');
        full_path.push_str(name);
        full_path
    }
}

impl era_solc::CollectableError for Build {
    fn errors(&self) -> Vec<&era_solc::StandardJsonOutputError> {
        let mut errors: Vec<&era_solc::StandardJsonOutputError> = self
            .results
            .values()
            .filter_map(|build| build.as_ref().err())
            .collect();
        errors.extend(
            self.messages
                .iter()
                .filter(|message| message.severity == "error"),
        );
        errors
    }

    fn take_warnings(&mut self) -> Vec<era_solc::StandardJsonOutputError> {
        let mut warnings: Vec<era_solc::StandardJsonOutputError> = self
            .messages
            .iter()
            .filter(|message| message.severity == "warning")
            .cloned()
            .collect();
        for contract in self.results.values_mut().flatten() {
            warnings.extend(
                contract
                    .deploy_object
                    .warnings
                    .drain(..)
                    .chain(contract.runtime_object.warnings.drain(..))
                    .map(|warning| {
                        era_solc::StandardJsonOutputError::new_warning(
                            warning,
                            Some(era_solc::StandardJsonOutputErrorSourceLocation::new(
                                contract.name.path.to_owned(),
                            )),
                            None,
                        )
                    }),
            );
        }
        self.messages
            .retain(|message| message.severity != "warning");
        warnings
    }
}
