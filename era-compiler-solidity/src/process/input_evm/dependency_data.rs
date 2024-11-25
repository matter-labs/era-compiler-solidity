//!
//! The EVM dependency data.
//!

use std::collections::BTreeMap;

///
/// The EVM dependency data.
///
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DependencyData {
    /// The `solc` compiler version.
    pub solc_version: Option<era_solc::Version>,
    /// The mapping of auxiliary identifiers, e.g. Yul object names, to full contract paths.
    pub identifier_paths: BTreeMap<String, String>,
}

impl DependencyData {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        solc_version: Option<era_solc::Version>,
        identifier_paths: BTreeMap<String, String>,
    ) -> Self {
        Self {
            solc_version,
            identifier_paths,
        }
    }
}

impl era_compiler_llvm_context::Dependency for DependencyData {
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
}
