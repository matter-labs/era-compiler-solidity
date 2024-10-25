//!
//! The `solc --standard-json` output selection.
//!

pub mod file;

use std::collections::HashSet;

use crate::solc::standard_json::input::settings::codegen::Codegen as SolcStandardJsonInputSettingsCodegen;

use self::file::flag::Flag as SelectionFlag;
use self::file::File as FileSelection;

///
/// The `solc --standard-json` output selection.
///
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Selection {
    /// Only the 'all' wildcard is available for robustness reasons.
    #[serde(default, rename = "*", skip_serializing_if = "FileSelection::is_empty")]
    pub all: FileSelection,
}

impl Selection {
    ///
    /// Creates the selection required by EraVM compilation process.
    ///
    pub fn new_required(codegen: SolcStandardJsonInputSettingsCodegen) -> Self {
        Self {
            all: FileSelection::new_required(codegen),
        }
    }

    ///
    /// Creates the selection required by Yul validation process.
    ///
    pub fn new_yul_validation() -> Self {
        Self {
            all: FileSelection::new_yul_validation(),
        }
    }

    ///
    /// Creates the selection for EraVM assembly.
    ///
    pub fn new_eravm_assembly() -> Self {
        Self {
            all: FileSelection::new_eravm_assembly(),
        }
    }

    ///
    /// Extends the output selection with flag required by EraVM compilation process.
    ///
    pub fn extend_with_required(
        &mut self,
        codegen: SolcStandardJsonInputSettingsCodegen,
    ) -> &mut Self {
        self.all.extend_with_required(codegen);
        self
    }

    ///
    /// Extends the output selection with flag required by the Yul validation.
    ///
    pub fn extend_with_yul_validation(&mut self) -> &mut Self {
        self.all.extend_with_yul_validation();
        self
    }

    ///
    /// Extends the output selection with EraVM assembly.
    ///
    pub fn extend_with_eravm_assembly(&mut self) -> &mut Self {
        self.all.extend_with_eravm_assembly();
        self
    }

    ///
    /// Returns flags that are going to be automatically added by the compiler,
    /// but were not explicitly requested by the user.
    ///
    /// Afterwards, the flags are used to prune JSON output before returning it.
    ///
    pub fn get_unset_required(&self) -> HashSet<SelectionFlag> {
        self.all.get_unset_required()
    }

    ///
    /// Whether EraVM assembly is requested.
    ///
    pub fn contains_eravm_assembly(&self) -> bool {
        self.all
            .per_contract
            .contains(&SelectionFlag::EraVMAssembly)
    }
}
