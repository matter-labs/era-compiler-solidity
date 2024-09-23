//!
//! The Solidity libraries.
//!

use std::collections::BTreeMap;
use std::collections::HashSet;

use crate::solc::standard_json::output::Output as StandardJsonOutput;
use crate::solc::version::Version as SolcVersion;

///
/// The Solidity libraries.
///
pub struct Libraries {
    /// The libraries.
    pub contract_libraries: BTreeMap<String, HashSet<String>>,
}

impl Libraries {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(contract_libraries: BTreeMap<String, HashSet<String>>) -> Self {
        Self { contract_libraries }
    }

    ///
    /// Writes the missing libraries to the standard JSON.
    ///
    pub fn write_to_standard_json(
        mut self,
        standard_json: &mut StandardJsonOutput,
        solc_version: Option<&SolcVersion>,
        zksolc_version: &semver::Version,
    ) -> anyhow::Result<()> {
        let contracts = match standard_json.contracts.as_mut() {
            Some(contracts) => contracts,
            None => return Ok(()),
        };

        for (path, contracts) in contracts.iter_mut() {
            for (name, contract) in contracts.iter_mut() {
                let full_name = format!("{path}:{name}");
                let missing_libraries = self.contract_libraries.remove(full_name.as_str());

                if let Some(missing_libraries) = missing_libraries {
                    contract.missing_libraries = Some(missing_libraries);
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
    /// Parses the library list and returns their double hashmap with path and name as keys.
    ///
    pub fn into_standard_json(
        libraries: Vec<String>,
    ) -> anyhow::Result<BTreeMap<String, BTreeMap<String, String>>> {
        let mut result = BTreeMap::new();
        for (index, library) in libraries.into_iter().enumerate() {
            let mut path_and_address = library.split('=');
            let path = path_and_address
                .next()
                .ok_or_else(|| anyhow::anyhow!("The library #{} path is missing", index))?;
            let mut file_and_contract = path.split(':');
            let file = file_and_contract
                .next()
                .ok_or_else(|| anyhow::anyhow!("The library `{}` file name is missing", path))?;
            let contract = file_and_contract.next().ok_or_else(|| {
                anyhow::anyhow!("The library `{}` contract name is missing", path)
            })?;
            let address = path_and_address
                .next()
                .ok_or_else(|| anyhow::anyhow!("The library `{}` address is missing", path))?;
            result
                .entry(file.to_owned())
                .or_insert_with(BTreeMap::new)
                .insert(contract.to_owned(), address.to_owned());
        }
        Ok(result)
    }

    ///
    /// Parses the library list and returns a mapping of library paths to their addresses.
    ///
    pub fn into_linker(
        libraries: Vec<String>,
    ) -> anyhow::Result<BTreeMap<String, [u8; era_compiler_common::BYTE_LENGTH_ETH_ADDRESS]>> {
        let mut result = BTreeMap::new();
        for (index, library) in libraries.into_iter().enumerate() {
            let mut path_and_address = library.split('=');
            let path = path_and_address
                .next()
                .ok_or_else(|| anyhow::anyhow!("The library #{} path is missing", index))?;

            let address = path_and_address
                .next()
                .ok_or_else(|| anyhow::anyhow!("The library `{}` address is missing", path))?;
            let address: [u8; era_compiler_common::BYTE_LENGTH_ETH_ADDRESS] =
                hex::decode(address.strip_prefix("0x").unwrap_or(address))
                    .map_err(|error| {
                        anyhow::anyhow!("Invalid address of library `{path}`: {error}")
                    })
                    .and_then(|address| {
                        address.try_into().map_err(|address: Vec<u8>| {
                            anyhow::anyhow!(
                                "Invalid address size of library `{path}`: expected {}, found {}",
                                era_compiler_common::BYTE_LENGTH_ETH_ADDRESS,
                                address.len(),
                            )
                        })
                    })?;

            result.insert(path.to_owned(), address);
        }
        Ok(result)
    }
}
