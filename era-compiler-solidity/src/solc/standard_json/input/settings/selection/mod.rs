//!
//! The `solc --standard-json` output selection.
//!

pub mod file;

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
    /// Creates the selection with arbitrary flags.
    ///
    pub fn new(flags: Vec<SelectionFlag>) -> Self {
        Self {
            all: FileSelection::new(flags),
        }
    }

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
    /// Extends the output selection with another one.
    ///
    pub fn extend(&mut self, other: Self) -> &mut Self {
        self.all.extend(other.all);
        self
    }

    ///
    /// Returns flags that are going to be automatically added by the compiler,
    /// but were not explicitly requested by the user.
    ///
    /// Afterwards, the flags are used to prune JSON output before returning it.
    ///
    pub fn selection_to_prune(&self) -> Self {
        Self {
            all: self.all.selection_to_prune(),
        }
    }

    ///
    /// Whether the flag is requested.
    ///
    pub fn contains(&self, flag: &SelectionFlag) -> bool {
        self.all.contains(flag)
    }
}
