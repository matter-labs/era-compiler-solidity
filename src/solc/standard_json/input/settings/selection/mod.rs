//!
//! The `solc --standard-json` output selection.
//!

pub mod file;

use serde::Deserialize;
use serde::Serialize;

use crate::solc::pipeline::Pipeline as SolcPipeline;

use self::file::File as FileSelection;

///
/// The `solc --standard-json` output selection.
///
#[derive(Debug, Default, Serialize, Deserialize)]
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
}
