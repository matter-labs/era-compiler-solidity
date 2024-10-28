//!
//! The `solc --standard-json` output file selection.
//!

pub mod flag;

use std::collections::HashSet;

use crate::solc::standard_json::input::settings::codegen::Codegen as SolcStandardJsonInputSettingsCodegen;

use self::flag::Flag as SelectionFlag;

///
/// The `solc --standard-json` output file selection.
///
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct File {
    /// The per-file output selections.
    #[serde(rename = "", skip_serializing_if = "HashSet::is_empty")]
    pub per_file: HashSet<SelectionFlag>,
    /// The per-contract output selections.
    #[serde(rename = "*", skip_serializing_if = "HashSet::is_empty")]
    pub per_contract: HashSet<SelectionFlag>,
}

impl File {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(flags: Vec<SelectionFlag>) -> Self {
        let mut per_file = HashSet::new();
        let mut per_contract = HashSet::new();
        for flag in flags.into_iter() {
            match flag {
                SelectionFlag::AST => {
                    per_file.insert(SelectionFlag::AST);
                }
                flag => {
                    per_contract.insert(flag);
                }
            }
        }
        Self {
            per_file,
            per_contract,
        }
    }

    ///
    /// Creates the selection required by EraVM compilation process.
    ///
    pub fn new_required(codegen: SolcStandardJsonInputSettingsCodegen) -> Self {
        Self::new(vec![
            SelectionFlag::AST,
            SelectionFlag::MethodIdentifiers,
            SelectionFlag::Metadata,
            codegen.into(),
        ])
    }

    ///
    /// Creates the selection required by Yul validation process.
    ///
    pub fn new_yul_validation() -> Self {
        Self::new(vec![SelectionFlag::EVM])
    }

    ///
    /// Extends the output selection with another one.
    ///
    pub fn extend(&mut self, other: Self) -> &mut Self {
        self.per_file.extend(other.per_file);
        self.per_contract.extend(other.per_contract);
        self
    }

    ///
    /// Returns flags that are going to be automatically added by the compiler,
    /// but were not explicitly requested by the user.
    ///
    /// Afterwards, the flags are used to prune JSON output before returning it.
    ///
    pub fn selection_to_prune(&self) -> Self {
        let required_per_file = vec![SelectionFlag::AST];
        let required_per_contract = vec![
            SelectionFlag::MethodIdentifiers,
            SelectionFlag::Metadata,
            SelectionFlag::Yul,
            SelectionFlag::EVMLA,
        ];

        let mut unset_per_file = HashSet::with_capacity(required_per_file.len());
        let mut unset_per_contract = HashSet::with_capacity(required_per_contract.len());

        for flag in required_per_file {
            if !self.per_file.contains(&flag) {
                unset_per_file.insert(flag);
            }
        }
        for flag in required_per_contract {
            if !self.per_contract.contains(&flag) {
                unset_per_contract.insert(flag);
            }
        }
        Self {
            per_file: unset_per_file,
            per_contract: unset_per_contract,
        }
    }

    ///
    /// Whether the flag is requested.
    ///
    pub fn contains(&self, flag: &SelectionFlag) -> bool {
        match flag {
            flag @ SelectionFlag::AST => self.per_file.contains(flag),
            flag => self.per_contract.contains(flag),
        }
    }

    ///
    /// Checks whether the selection is empty.
    ///
    pub fn is_empty(&self) -> bool {
        self.per_file.is_empty() && self.per_contract.is_empty()
    }
}
