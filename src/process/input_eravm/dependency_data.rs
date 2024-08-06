//!
//! The EraVM dependency data.
//!

use std::collections::BTreeMap;

use crate::build_eravm::contract::Contract as EraVMContractBuild;
use crate::solc::version::Version as SolcVersion;

///
/// The EraVM dependency data.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DependencyData {
    /// The `solc` compiler version.
    pub solc_version: Option<SolcVersion>,
    /// The mapping of auxiliary identifiers, e.g. Yul object names, to full contract paths.
    pub identifier_paths: BTreeMap<String, String>,
    /// The library addresses.
    pub libraries: BTreeMap<String, BTreeMap<String, String>>,
    /// The dependencies required by specific contract.
    pub dependencies: BTreeMap<String, EraVMContractBuild>,
}

impl DependencyData {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        solc_version: Option<SolcVersion>,
        identifier_paths: BTreeMap<String, String>,
        libraries: BTreeMap<String, BTreeMap<String, String>>,
    ) -> Self {
        Self {
            solc_version,
            identifier_paths,
            libraries,
            dependencies: BTreeMap::new(),
        }
    }
}

impl era_compiler_llvm_context::Dependency for DependencyData {
    fn get(&self, identifier: &str) -> anyhow::Result<String> {
        let path = self.resolve_path(identifier)?;
        let contract = self
            .dependencies
            .get(path.as_str())
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("dependency `{path}` not found in the project"))?;
        Ok(hex::encode(contract.build.bytecode_hash))
    }

    fn resolve_path(&self, identifier: &str) -> anyhow::Result<String> {
        self.identifier_paths
            .get(identifier.strip_suffix("_deployed").unwrap_or(identifier))
            .cloned()
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "dependency with identifier `{}` not found in the project",
                    identifier
                )
            })
    }

    fn resolve_library(&self, path: &str) -> anyhow::Result<String> {
        for (file_path, contracts) in self.libraries.iter() {
            for (contract_name, address) in contracts.iter() {
                let key = format!("{file_path}:{contract_name}");
                if key.as_str() == path {
                    return Ok(address["0x".len()..].to_owned());
                }
            }
        }

        anyhow::bail!("library `{path}` not found in the project");
    }
}
