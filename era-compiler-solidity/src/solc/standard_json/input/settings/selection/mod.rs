//!
//! The `solc --standard-json` output selection.
//!

pub mod file;

use std::collections::HashSet;

use crate::solc::pipeline::Pipeline as SolcPipeline;

use self::file::flag::Flag as SelectionFlag;
use self::file::File as FileSelection;

///
/// The `solc --standard-json` output selection.
///
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Selection {
    /// Only the 'all' wildcard is available for robustness reasons.
    #[serde(rename = "*", skip_serializing_if = "Option::is_none")]
    pub all: Option<FileSelection>,
}

impl Selection {
    ///
    /// Creates the selection required by EraVM compilation process.
    ///
    pub fn new_required(pipeline: Option<SolcPipeline>) -> Self {
        Self {
            all: Some(FileSelection::new_required(pipeline)),
        }
    }

    ///
    /// Creates the selection required by Yul validation process.
    ///
    pub fn new_yul_validation() -> Self {
        Self {
            all: Some(FileSelection::new_yul_validation()),
        }
    }

    ///
    /// Creates the selection for EraVM assembly.
    ///
    pub fn new_eravm_assembly() -> Self {
        Self {
            all: Some(FileSelection::new_eravm_assembly()),
        }
    }

    ///
    /// Extends the output selection with flag required by EraVM compilation process.
    ///
    pub fn extend_with_required(&mut self, pipeline: Option<SolcPipeline>) -> &mut Self {
        self.all
            .get_or_insert_with(|| FileSelection::new_required(pipeline))
            .extend_with_required(pipeline);
        self
    }

    ///
    /// Extends the output selection with flag required by the Yul validation.
    ///
    pub fn extend_with_yul_validation(&mut self) -> &mut Self {
        self.all
            .get_or_insert_with(FileSelection::new_yul_validation)
            .extend_with_yul_validation();
        self
    }

    ///
    /// Extends the output selection with EraVM assembly.
    ///
    pub fn extend_with_eravm_assembly(&mut self) -> &mut Self {
        self.all
            .get_or_insert_with(FileSelection::new_eravm_assembly)
            .extend_with_eravm_assembly();
        self
    }

    ///
    /// Returns flags that are going to be automatically added by the compiler,
    /// but were not explicitly requested by the user.
    ///
    /// Afterwards, the flags are used to prune JSON output before returning it.
    ///
    pub fn get_unset_required(&self) -> HashSet<SelectionFlag> {
        self.all
            .as_ref()
            .map(|selection| selection.get_unset_required())
            .unwrap_or_else(|| FileSelection::default().get_unset_required())
    }

    ///
    /// Whether EraVM assembly is requested.
    ///
    pub fn contains_eravm_assembly(&self) -> bool {
        self.all
            .as_ref()
            .and_then(|file| file.per_contract.as_ref())
            .map(|contract| contract.contains(&SelectionFlag::EraVMAssembly))
            .unwrap_or_default()
    }
}
