//!
//! The `solc --standard-json` output file selection.
//!

pub mod flag;

use std::collections::HashSet;

use crate::solc::pipeline::Pipeline as SolcPipeline;

use self::flag::Flag as SelectionFlag;

///
/// The `solc --standard-json` output file selection.
///
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
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
        let pipeline_ir_flag = match pipeline {
            Some(pipeline) => SelectionFlag::from(pipeline),
            None => SelectionFlag::Yul,
        };

        let per_file = HashSet::from_iter([SelectionFlag::AST]);
        let per_contract = HashSet::from_iter([
            SelectionFlag::MethodIdentifiers,
            SelectionFlag::Metadata,
            pipeline_ir_flag,
        ]);
        Self {
            per_file: Some(per_file),
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
    /// Creates the selection for EraVM assembly.
    ///
    pub fn new_eravm_assembly() -> Self {
        Self {
            per_file: Some(HashSet::new()),
            per_contract: Some(HashSet::from_iter([SelectionFlag::EraVMAssembly])),
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

    ///
    /// Extends the output selection with EraVM assembly flag.
    ///
    pub fn extend_with_eravm_assembly(&mut self) -> &mut Self {
        let eravm_assembly = Self::new_eravm_assembly();
        self.per_file
            .get_or_insert_with(HashSet::default)
            .extend(eravm_assembly.per_file.unwrap_or_default());
        self.per_contract
            .get_or_insert_with(HashSet::default)
            .extend(eravm_assembly.per_contract.unwrap_or_default());
        self
    }

    ///
    /// Returns flags that are going to be automatically added by the compiler,
    /// but were not explicitly requested by the user.
    ///
    /// Afterwards, the flags are used to prune JSON output before returning it.
    ///
    pub fn get_unset_required(&self) -> HashSet<SelectionFlag> {
        let required_per_file = vec![SelectionFlag::AST];
        let required_per_contract = vec![
            SelectionFlag::MethodIdentifiers,
            SelectionFlag::Metadata,
            SelectionFlag::Yul,
            SelectionFlag::EVMLA,
        ];
        let mut flags =
            HashSet::with_capacity(required_per_file.len() + required_per_contract.len());

        for flag in required_per_file {
            if !self
                .per_file
                .as_ref()
                .map(|per_file| per_file.contains(&flag))
                .unwrap_or_default()
            {
                flags.insert(flag);
            }
        }
        for flag in required_per_contract {
            if !self
                .per_contract
                .as_ref()
                .map(|per_contract| per_contract.contains(&flag))
                .unwrap_or_default()
            {
                flags.insert(flag);
            }
        }
        flags
    }
}
