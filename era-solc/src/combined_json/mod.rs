//!
//! The `solc --combined-json` output.
//!

pub mod contract;

use std::collections::BTreeMap;
use std::path::Path;

use self::contract::Contract;

///
/// The `solc --combined-json` output.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CombinedJson {
    /// The contract entries.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub contracts: BTreeMap<String, Contract>,
    /// The list of source files.
    #[serde(default, rename = "sourceList", skip_serializing_if = "Vec::is_empty")]
    pub source_list: Vec<String>,
    /// The source code extra data, including the AST.
    #[serde(default, skip_serializing_if = "serde_json::Value::is_null")]
    pub sources: serde_json::Value,
    /// The `solc` compiler version.
    pub version: String,
    /// The `zksolc` compiler version.
    #[serde(default = "crate::version")]
    pub zk_version: String,
}

impl CombinedJson {
    ///
    /// Writes the JSON to the specified directory.
    ///
    pub fn write_to_directory(
        self,
        output_directory: &Path,
        overwrite: bool,
    ) -> anyhow::Result<()> {
        let mut file_path = output_directory.to_owned();
        file_path.push(format!("combined.{}", era_compiler_common::EXTENSION_JSON));

        if file_path.exists() && !overwrite {
            anyhow::bail!(
                "Refusing to overwrite an existing file {file_path:?} (use --overwrite to force)."
            );
        }

        std::fs::write(
            file_path.as_path(),
            serde_json::to_vec(&self).expect("Always valid").as_slice(),
        )
        .map_err(|error| anyhow::anyhow!("File {file_path:?} writing: {error}"))?;

        Ok(())
    }

    ///
    /// Removes EVM artifacts to prevent their accidental usage.
    ///
    pub fn remove_evm(&mut self) {
        for (_, contract) in self.contracts.iter_mut() {
            contract.bin = None;
            contract.bin_runtime = None;
        }
    }
}
