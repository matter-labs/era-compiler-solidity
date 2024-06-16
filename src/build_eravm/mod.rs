//!
//! The Solidity project build.
//!

pub mod contract;

use std::collections::BTreeMap;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use crate::solc::combined_json::CombinedJson;
use crate::solc::standard_json::output::contract::Contract as StandardJsonOutputContract;
use crate::solc::standard_json::output::error::Error as StandardJsonOutputError;
use crate::solc::standard_json::output::Output as StandardJsonOutput;
use crate::solc::version::Version as SolcVersion;

use self::contract::Contract;

///
/// The Solidity project build.
///
#[derive(Debug)]
pub struct Build {
    /// The contract data,
    pub contracts: BTreeMap<String, Result<Contract, StandardJsonOutputError>>,
    /// The additional message to output.
    pub messages: Vec<StandardJsonOutputError>,
}

impl Build {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        contracts: BTreeMap<String, Result<Contract, StandardJsonOutputError>>,
        messages: &mut Vec<StandardJsonOutputError>,
    ) -> Self {
        Self {
            contracts,
            messages: std::mem::take(messages),
        }
    }

    ///
    /// Writes all contracts to the terminal.
    ///
    pub fn write_to_terminal(mut self, output_binary: bool) -> anyhow::Result<()> {
        self.take_and_write_warnings();
        self.collect_errors()?;

        for (path, build) in self.contracts.into_iter() {
            build
                .expect("Always valid")
                .write_to_terminal(path, output_binary)?;
        }

        Ok(())
    }

    ///
    /// Writes all contracts to the specified directory.
    ///
    pub fn write_to_directory(
        mut self,
        output_directory: &Path,
        output_binary: bool,
        overwrite: bool,
    ) -> anyhow::Result<()> {
        self.take_and_write_warnings();
        self.collect_errors()?;

        for build in self.contracts.into_values() {
            build.expect("Always valid").write_to_directory(
                output_directory,
                output_binary,
                overwrite,
            )?;
        }

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

        let mut errors = Vec::with_capacity(self.contracts.len());
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
                Err(error) => errors.push(error),
            }
        }

        standard_json
            .errors
            .get_or_insert_with(Vec::new)
            .extend(errors);
        if let Some(solc_version) = solc_version {
            standard_json.version = Some(solc_version.default.to_string());
            standard_json.long_version = Some(solc_version.long.to_owned());
        }
        standard_json.zk_version = Some(zksolc_version.to_string());

        Ok(())
    }

    ///
    /// Writes all contracts assembly and bytecode to the combined JSON.
    ///
    pub fn write_to_combined_json(
        mut self,
        combined_json: &mut CombinedJson,
        zksolc_version: &semver::Version,
    ) -> anyhow::Result<()> {
        self.take_and_write_warnings();
        self.collect_errors()?;

        for (path, build) in self.contracts.into_iter() {
            let combined_json_contract = combined_json
                .contracts
                .iter_mut()
                .find_map(|(json_path, contract)| {
                    let path = PathBuf::from(path.split(':').next().expect("Always exists"))
                        .canonicalize()
                        .expect("Path canonicalization error");
                    let json_path =
                        PathBuf::from(json_path.split(':').next().expect("Always exists"))
                            .canonicalize()
                            .expect("Path canonicalization error");

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

        combined_json.zk_version = Some(zksolc_version.to_string());

        Ok(())
    }

    ///
    /// Checks if there is at least one error.
    ///
    pub fn has_errors(&self) -> bool {
        self.contracts.values().any(|result| result.is_err())
            && self
                .messages
                .iter()
                .any(|message| message.severity == "error")
    }

    ///
    /// Checks if there is at least one warning.
    ///
    pub fn has_warnings(&self) -> bool {
        self.messages
            .iter()
            .any(|message| message.severity == "warning")
    }

    ///
    /// Checks for errors, returning `Err` if there is at least one error.
    ///
    pub fn collect_errors(&self) -> anyhow::Result<()> {
        if !self.has_errors() {
            return Ok(());
        }
        let errors: Vec<&StandardJsonOutputError> = self
            .contracts
            .values()
            .filter_map(|result| result.as_ref().err())
            .collect();
        anyhow::bail!(
            "{}",
            errors
                .iter()
                .map(|error| error.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        );
    }

    ///
    /// Removes warnings from the list of messages and prints them to stderr.
    ///
    pub fn take_and_write_warnings(&mut self) {
        if !self.has_warnings() {
            return;
        }
        writeln!(
            std::io::stderr(),
            "{}",
            self.messages
                .iter()
                .filter(|error| error.severity == "warning")
                .map(|error| error.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
        .expect("Stderr writing error");
        self.messages
            .retain(|message| message.severity != "warning");
    }
}
