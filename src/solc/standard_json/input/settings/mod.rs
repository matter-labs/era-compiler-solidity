//!
//! The `solc --standard-json` input settings representation.
//!

pub mod optimizer;
pub mod selection;

use std::collections::BTreeMap;

use serde::Deserialize;
use serde::Serialize;

use self::optimizer::Optimizer;
use self::selection::Selection;

///
/// The `solc --standard-json` input settings representation.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    /// The linker library addresses.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub libraries: Option<BTreeMap<String, BTreeMap<String, String>>>,
    /// The output selection filters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_selection: Option<Selection>,
    /// The optimizer settings.
    pub optimizer: Optimizer,
}

impl Settings {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        libraries: BTreeMap<String, BTreeMap<String, String>>,
        output_selection: Selection,
        optimizer: Optimizer,
    ) -> Self {
        Self {
            libraries: Some(libraries),
            output_selection: Some(output_selection),
            optimizer,
        }
    }

    ///
    /// Parses the library list and returns their double hashmap with path and name as keys.
    ///
    pub fn parse_libraries(
        input: Vec<String>,
    ) -> anyhow::Result<BTreeMap<String, BTreeMap<String, String>>> {
        let mut libraries = BTreeMap::new();
        for (index, library) in input.into_iter().enumerate() {
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
            libraries
                .entry(file.to_owned())
                .or_insert_with(BTreeMap::new)
                .insert(contract.to_owned(), address.to_owned());
        }
        Ok(libraries)
    }
}
