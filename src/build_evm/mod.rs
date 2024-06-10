//!
//! The Solidity project build.
//!

pub mod contract;

use std::collections::BTreeMap;
use std::path::Path;

use crate::solc::combined_json::CombinedJson;
use crate::solc::standard_json::output::contract::Contract as StandardJsonOutputContract;
use crate::solc::standard_json::output::error::source_location::SourceLocation as StandardJsonOutputErrorSourceLocation;
use crate::solc::standard_json::output::error::Error as StandardJsonOutputError;
use crate::solc::standard_json::output::Output as StandardJsonOutput;
use crate::solc::version::Version as SolcVersion;

use self::contract::Contract;

///
/// The Solidity project build.
///
#[derive(Debug, Default)]
pub struct Build {
    /// The contract data,
    pub contracts: BTreeMap<String, anyhow::Result<Contract>>,
}

impl Build {
    ///
    /// Writes all contracts to the terminal.
    ///
    pub fn write_to_terminal(
        self,
        output_assembly: bool,
        output_binary: bool,
    ) -> anyhow::Result<()> {
        self.check_errors()?;

        for (path, build) in self.contracts.into_iter() {
            build
                .expect("Always valid")
                .write_to_terminal(path, output_assembly, output_binary)?;
        }

        Ok(())
    }

    ///
    /// Writes all contracts to the specified directory.
    ///
    pub fn write_to_directory(
        self,
        output_directory: &Path,
        output_assembly: bool,
        output_binary: bool,
        overwrite: bool,
    ) -> anyhow::Result<()> {
        self.check_errors()?;

        for build in self.contracts.into_values() {
            build.expect("Always valid").write_to_directory(
                output_directory,
                output_assembly,
                output_binary,
                overwrite,
            )?;
        }

        Ok(())
    }

    ///
    /// Writes all contracts assembly and bytecode to the combined JSON.
    ///
    pub fn write_to_combined_json(
        self,
        combined_json: &mut CombinedJson,
        zksolc_version: &semver::Version,
    ) -> anyhow::Result<()> {
        self.check_errors()?;

        for (path, build) in self.contracts.into_iter() {
            let combined_json_contract = combined_json
                .contracts
                .iter_mut()
                .find_map(|(json_path, contract)| {
                    if path.ends_with(json_path) {
                        Some(contract)
                    } else {
                        None
                    }
                })
                .ok_or_else(|| anyhow::anyhow!("Contract `{}` not found in the project", path))?;

            build
                .expect("Always valid")
                .write_to_combined_json(combined_json_contract)?;
        }

        combined_json.zk_version = Some(zksolc_version.to_string());

        Ok(())
    }

    ///
    /// Writes all contracts assembly and bytecode to the standard JSON.
    ///
    pub fn write_to_standard_json(
        self,
        standard_json: &mut StandardJsonOutput,
        solc_version: Option<&SolcVersion>,
        zksolc_version: &semver::Version,
    ) -> anyhow::Result<()> {
        let standard_json_contracts = match standard_json.contracts.as_mut() {
            Some(contracts) => contracts,
            None => {
                standard_json.contracts = Some(BTreeMap::new());
                standard_json.contracts.as_mut().expect("Always exists")
            }
        };
        let standard_json_errors = match standard_json.errors.as_mut() {
            Some(errors) => errors,
            None => {
                standard_json.errors = Some(Vec::new());
                standard_json.errors.as_mut().expect("Always exists")
            }
        };

        for (full_path, build) in self.contracts.into_iter() {
            let mut full_path_split = full_path.split(':');
            let path = full_path_split.next().expect("Always exists");
            let name = full_path_split.next().unwrap_or(path);

            match build {
                Ok(build) => match standard_json_contracts
                    .get_mut(path)
                    .and_then(|contracts| contracts.get_mut(name))
                {
                    Some(contract) => {
                        build.write_to_standard_json(contract)?;
                    }
                    None => {
                        let contracts = standard_json_contracts
                            .entry(path.to_owned())
                            .or_insert_with(BTreeMap::new);
                        let mut contract = StandardJsonOutputContract::default();
                        build.write_to_standard_json(&mut contract)?;
                        contracts.insert(name.to_owned(), contract);
                    }
                },
                Err(error) => {
                    let message = error.to_string();
                    standard_json_errors.push(StandardJsonOutputError {
                        component: "general".to_owned(),
                        error_code: None,
                        formatted_message: message.clone(),
                        message,
                        severity: "error".to_owned(),
                        source_location: Some(StandardJsonOutputErrorSourceLocation::new(
                            path.to_owned(),
                            0,
                            0,
                        )),
                        r#type: "Error".to_owned(),
                    });
                }
            }
        }

        if let Some(solc_version) = solc_version {
            standard_json.version = Some(solc_version.default.to_string());
            standard_json.long_version = Some(solc_version.long.to_owned());
        }
        standard_json.zk_version = Some(zksolc_version.to_string());

        Ok(())
    }

    ///
    /// Checks for errors, returning `Err` if there is at least one error.
    ///
    pub fn check_errors(&self) -> anyhow::Result<()> {
        let mut errors = Vec::new();
        for (path, contract) in self.contracts.iter() {
            if let Err(ref error) = contract {
                errors.push((path.to_owned(), error.to_string()));
            }
        }
        if !errors.is_empty() {
            anyhow::bail!(
                "{}",
                errors
                    .iter()
                    .map(|(path, error)| format!("Contract `{path}` error: {error}"))
                    .collect::<Vec<String>>()
                    .join("\n")
            );
        }

        Ok(())
    }
}
