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
    /// Creates the selection for EraVM assembly.
    ///
    pub fn new_eravm_assembly() -> Self {
        Self::new(vec![SelectionFlag::EraVMAssembly])
    }

    ///
    /// Extends the output selection with flag required by EraVM compilation process.
    ///
    pub fn extend_with_required(
        &mut self,
        codegen: SolcStandardJsonInputSettingsCodegen,
    ) -> &mut Self {
        let required = Self::new_required(codegen);
        self.per_file.extend(required.per_file);
        self.per_contract.extend(required.per_contract);
        self
    }

    ///
    /// Extends the output selection with flag required by the Yul validation.
    ///
    pub fn extend_with_yul_validation(&mut self) -> &mut Self {
        let yul_validation = Self::new_yul_validation();
        self.per_file.extend(yul_validation.per_file);
        self.per_contract.extend(yul_validation.per_contract);
        self
    }

    ///
    /// Extends the output selection with EraVM assembly flag.
    ///
    pub fn extend_with_eravm_assembly(&mut self) -> &mut Self {
        let eravm_assembly = Self::new_eravm_assembly();
        self.per_file.extend(eravm_assembly.per_file);
        self.per_contract.extend(eravm_assembly.per_contract);
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
            if !self.per_file.contains(&flag) {
                flags.insert(flag);
            }
        }
        for flag in required_per_contract {
            if !self.per_contract.contains(&flag) {
                flags.insert(flag);
            }
        }
        flags
    }

    ///
    /// Checks whether the selection is empty.
    ///
    pub fn is_empty(&self) -> bool {
        self.per_file.is_empty() && self.per_contract.is_empty()
    }
}
