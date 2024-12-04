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
            build.expect("Always valid").write_to_terminal(
                path,
                output_metadata,
                output_assembly,
                output_binary,
            )?;
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
        output_assembly: bool,
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
                output_assembly,
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
                .ok_or_else(|| anyhow::anyhow!("Contract `{path}` not found in the project"))?;

            build
                .expect("Always valid")
                .write_to_combined_json(combined_json_contract)?;
        }

        Ok(())
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
        let warnings = self
            .messages
            .iter()
            .filter(|message| message.severity == "warning")
            .cloned()
            .collect();
        self.messages
            .retain(|message| message.severity != "warning");
        warnings
    }
}
