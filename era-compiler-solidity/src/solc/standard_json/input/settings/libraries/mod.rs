//!
//! The Solidity libraries.
//!

pub mod missing;

use std::collections::BTreeMap;

///
/// The Solidity libraries.
///
#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Libraries {
    /// The unified representation of libraries.
    #[serde(flatten)]
    pub inner: BTreeMap<String, BTreeMap<String, String>>,
}

impl Libraries {
    ///
    /// Returns a representation of libraries suitable for the LLVM linker.
    ///
    pub fn as_linker_symbols(
        &self,
    ) -> anyhow::Result<BTreeMap<String, [u8; era_compiler_common::BYTE_LENGTH_ETH_ADDRESS]>> {
        let mut linker_symbols = BTreeMap::new();
        for (file, contracts) in self.inner.iter() {
            for (name, address) in contracts.iter() {
                let path = format!("{file}:{name}");

                let address_stripped = address.strip_prefix("0x").unwrap_or(address.as_str());
                let address_vec = hex::decode(address_stripped).map_err(|error| {
                    anyhow::anyhow!("Invalid address `{address}` of library `{path}`: {error}.")
                })?;
                let address_array: [u8; era_compiler_common::BYTE_LENGTH_ETH_ADDRESS] = address_vec.try_into().map_err(|address_vec: Vec<u8>| {
                    anyhow::anyhow!(
                        "Incorrect size of address `{address}` of library `{path}`: expected {}, found {}.",
                        era_compiler_common::BYTE_LENGTH_ETH_ADDRESS,
                        address_vec.len(),
                    )
                })?;

                linker_symbols.insert(path, address_array);
            }
        }
        Ok(linker_symbols)
    }

    ///
    /// Checks whether the libraries are empty.
    ///
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    ///
    /// Returns the reference to the inner value.
    ///
    pub fn as_inner(&self) -> &BTreeMap<String, BTreeMap<String, String>> {
        &self.inner
    }

    ///
    /// Extracts the inner value.
    ///
    pub fn into_inner(self) -> BTreeMap<String, BTreeMap<String, String>> {
        self.inner
    }
}

impl From<BTreeMap<String, BTreeMap<String, String>>> for Libraries {
    fn from(inner: BTreeMap<String, BTreeMap<String, String>>) -> Self {
        Self { inner }
    }
}

impl TryFrom<&[String]> for Libraries {
    type Error = anyhow::Error;

    fn try_from(arguments: &[String]) -> Result<Self, Self::Error> {
        let mut libraries = BTreeMap::new();
        for (index, library) in arguments.iter().enumerate() {
            let mut path_and_address = library.split('=');
            let path = path_and_address
                .next()
                .ok_or_else(|| anyhow::anyhow!("Library #{} path is missing.", index))?;
            let mut file_and_contract = path.split(':');
            let file = file_and_contract
                .next()
                .ok_or_else(|| anyhow::anyhow!("Library `{}` file name is missing.", path))?;
            let contract = file_and_contract
                .next()
                .ok_or_else(|| anyhow::anyhow!("Library `{}` contract name is missing.", path))?;
            let address = path_and_address
                .next()
                .ok_or_else(|| anyhow::anyhow!("Library `{}` address is missing.", path))?;
            libraries
                .entry(file.to_owned())
                .or_insert_with(BTreeMap::new)
                .insert(contract.to_owned(), address.to_owned());
        }
        Ok(Self { inner: libraries })
    }
}
