//!
//! The `solc --standard-json` output file selection.
//!

pub mod flag;

use std::collections::HashSet;

use serde::Deserialize;
use serde::Serialize;

use crate::solc::pipeline::Pipeline as SolcPipeline;

use self::flag::Flag as SelectionFlag;

///
/// The `solc --standard-json` output file selection.
///
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct File {
    /// The per-file output selections.
    #[serde(rename = "", skip_serializing_if = "Option::is_none")]
    pub per_file: Option<HashSet<SelectionFlag>>,
    /// The per-contract output selections.
    #[serde(rename = "*", skip_serializing_if = "Option::is_none")]
    pub per_contract: Option<HashSet<SelectionFlag>>,
}

impl File {
    ///
    /// Creates the selection required by EraVM compilation process.
    ///
    pub fn new_required(pipeline: Option<SolcPipeline>) -> Self {
        let mut per_contract =
            HashSet::from_iter([SelectionFlag::MethodIdentifiers, SelectionFlag::Metadata]);
        per_contract.insert(match pipeline {
            Some(pipeline) => SelectionFlag::from(pipeline),
            None => SelectionFlag::Yul,
        });
        Self {
            per_file: Some(HashSet::from_iter([SelectionFlag::AST])),
            per_contract: Some(per_contract),
        }
    }

    ///
    /// Creates the selection required by Yul validation process.
    ///
    pub fn new_yul_validation() -> Self {
        Self {
            per_file: Some(HashSet::new()),
            per_contract: Some(HashSet::from_iter([SelectionFlag::EVM])),
        }
    }

    ///
    /// Extends the output selection with flag required by EraVM compilation process.
    ///
    pub fn extend_with_required(&mut self, pipeline: Option<SolcPipeline>) -> &mut Self {
        let required = Self::new_required(pipeline);
        self.per_file
            .get_or_insert_with(HashSet::default)
            .extend(required.per_file.unwrap_or_default());
        self.per_contract
            .get_or_insert_with(HashSet::default)
            .extend(required.per_contract.unwrap_or_default());
        self
    }

    ///
    /// Extends the output selection with flag required by the Yul validation.
    ///
    pub fn extend_with_yul_validation(&mut self) -> &mut Self {
        let yul_validation = Self::new_yul_validation();
        self.per_file
            .get_or_insert_with(HashSet::default)
            .extend(yul_validation.per_file.unwrap_or_default());
        self.per_contract
            .get_or_insert_with(HashSet::default)
            .extend(yul_validation.per_contract.unwrap_or_default());
        self
    }
}
