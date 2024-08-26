//!
//! The Solidity project build.
//!

pub mod contract;

use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;

use normpath::PathExt;

use crate::solc::combined_json::CombinedJson;
use crate::solc::standard_json::output::contract::Contract as StandardJsonOutputContract;
use crate::solc::standard_json::output::error::collectable::Collectable as CollectableError;
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
        self.exit_on_error();

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
        self.exit_on_error();

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
        let standard_json_contracts = standard_json.contracts.get_or_insert_with(BTreeMap::new);
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
                        let contracts = standard_json_contracts.entry(path.to_owned()).or_default();
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
        self.exit_on_error();

        for (path, build) in self.contracts.into_iter() {
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

        combined_json.zk_version = Some(zksolc_version.to_string());

        Ok(())
    }
}

impl CollectableError for Build {
    fn errors(&self) -> Vec<&StandardJsonOutputError> {
        self.contracts
            .values()
            .filter_map(|build| build.as_ref().err())
            .collect()
    }

    fn warnings(&self) -> Vec<&StandardJsonOutputError> {
        self.messages.iter().collect()
    }

    fn remove_warnings(&mut self) {
        self.messages.clear();
    }
}
