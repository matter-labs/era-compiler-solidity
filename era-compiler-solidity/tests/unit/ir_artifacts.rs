//!
//! The Solidity compiler unit tests for IR artifacts.
//!
//! The tests check if the IR artifacts are kept in the final output.
//!

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use era_compiler_solidity::solc::standard_json::input::settings::codegen::Codegen as SolcStandardJsonInputSettingsCodegen;
use era_compiler_solidity::solc::standard_json::input::settings::libraries::Libraries;
use era_compiler_solidity::solc::Compiler as SolcCompiler;

use crate::common;

#[test]
#[cfg_attr(target_os = "windows", ignore)]
fn default_04_evmla() {
    evmla(semver::Version::new(0, 4, 26));
}
#[test]
#[cfg_attr(target_os = "windows", ignore)]
fn default_05_evmla() {
    evmla(semver::Version::new(0, 5, 17));
}
#[test]
fn default_06_evmla() {
    evmla(semver::Version::new(0, 6, 12));
}
#[test]
fn default_07_evmla() {
    evmla(semver::Version::new(0, 7, 6));
}
#[test]
fn default_08_evmla() {
    evmla(SolcCompiler::LAST_SUPPORTED_VERSION);
}
#[test]
fn default_08_yul() {
    yul(SolcCompiler::LAST_SUPPORTED_VERSION);
}

pub const SOURCE_CODE: &str = r#"
// SPDX-License-Identifier: MIT
pragma solidity >=0.4.12;

contract Test {
    function main() public pure returns (uint) {
        return 42;
    }
}
"#;

fn yul(version: semver::Version) {
    let mut sources = BTreeMap::new();
    sources.insert("test.sol".to_owned(), SOURCE_CODE.to_owned());

    let build = common::build_solidity(
        sources.clone(),
        Libraries::default(),
        era_compiler_common::HashType::Ipfs,
        BTreeSet::new(),
        &version,
        SolcStandardJsonInputSettingsCodegen::Yul,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");

    assert!(
        !build
            .contracts
            .get("test.sol")
            .expect("Always exists")
            .get("Test")
            .expect("Always exists")
            .ir_optimized
            .is_empty(),
        "Yul is missing"
    );
    assert!(
        build
            .contracts
            .get("test.sol")
            .expect("Always exists")
            .get("Test")
            .expect("Always exists")
            .evm
            .as_ref()
            .expect("EVM object is missing")
            .legacy_assembly
            .is_none(),
        "EVM assembly is present although not requested"
    );
}

fn evmla(version: semver::Version) {
    let mut sources = BTreeMap::new();
    sources.insert("test.sol".to_owned(), SOURCE_CODE.to_owned());

    let build = common::build_solidity(
        sources.clone(),
        Libraries::default(),
        era_compiler_common::HashType::Ipfs,
        BTreeSet::new(),
        &version,
        SolcStandardJsonInputSettingsCodegen::EVMLA,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");
    assert!(
        build
            .contracts
            .get("test.sol")
            .expect("Always exists")
            .get("Test")
            .expect("Always exists")
            .evm
            .as_ref()
            .expect("EVM object is missing")
            .legacy_assembly
            .is_some(),
        "EVM assembly is missing",
    );
    assert!(
        build
            .contracts
            .get("test.sol")
            .expect("Always exists")
            .get("Test")
            .expect("Always exists")
            .ir_optimized
            .is_empty(),
        "Yul is present although not requested",
    );
}
