//!
//! The Solidity compiler unit tests for libraries.
//!

use std::collections::BTreeMap;

use crate::common;

#[test]
#[cfg_attr(target_os = "windows", ignore)]
fn not_specified_04_evmla() {
    not_specified(
        semver::Version::new(0, 4, 26),
        era_solc::StandardJsonInputCodegen::EVMLA,
    );
}
#[test]
#[cfg_attr(target_os = "windows", ignore)]
fn not_specified_05_evmla() {
    not_specified(
        semver::Version::new(0, 5, 17),
        era_solc::StandardJsonInputCodegen::EVMLA,
    );
}
#[test]
fn not_specified_06_evmla() {
    not_specified(
        semver::Version::new(0, 6, 12),
        era_solc::StandardJsonInputCodegen::EVMLA,
    );
}
#[test]
fn not_specified_07_evmla() {
    not_specified(
        semver::Version::new(0, 7, 6),
        era_solc::StandardJsonInputCodegen::EVMLA,
    );
}
#[test]
fn not_specified_08_evmla() {
    not_specified(
        era_solc::Compiler::LAST_SUPPORTED_VERSION,
        era_solc::StandardJsonInputCodegen::EVMLA,
    );
}
#[test]
fn not_specified_08_yul() {
    not_specified(
        era_solc::Compiler::LAST_SUPPORTED_VERSION,
        era_solc::StandardJsonInputCodegen::Yul,
    );
}

#[test]
#[cfg_attr(target_os = "windows", ignore)]
fn specified_04_evmla() {
    specified(
        semver::Version::new(0, 4, 26),
        era_solc::StandardJsonInputCodegen::EVMLA,
    );
}
#[test]
#[cfg_attr(target_os = "windows", ignore)]
fn specified_05_evmla() {
    specified(
        semver::Version::new(0, 5, 17),
        era_solc::StandardJsonInputCodegen::EVMLA,
    );
}
#[test]
fn specified_06_evmla() {
    specified(
        semver::Version::new(0, 6, 12),
        era_solc::StandardJsonInputCodegen::EVMLA,
    );
}
#[test]
fn specified_07_evmla() {
    specified(
        semver::Version::new(0, 7, 6),
        era_solc::StandardJsonInputCodegen::EVMLA,
    );
}
#[test]
fn specified_08_evmla() {
    specified(
        era_solc::Compiler::LAST_SUPPORTED_VERSION,
        era_solc::StandardJsonInputCodegen::EVMLA,
    );
}
#[test]
fn specified_08_yul() {
    specified(
        era_solc::Compiler::LAST_SUPPORTED_VERSION,
        era_solc::StandardJsonInputCodegen::Yul,
    );
}

pub const LIBRARY_TEST_SOURCE: &str = r#"
// SPDX-License-Identifier: MIT
pragma solidity >=0.4.12;

// A simple library with at least one external method
library SimpleLibrary {
    function add(uint256 a, uint256 b) external pure returns (uint256) {
        return a + b;
    }
}

// A contract calling that library
contract SimpleContract {
    using SimpleLibrary for uint256;

    function performAlgorithm(uint256 a, uint256 b) public pure returns (uint256) {
        uint sum = 0;
        if (a > b) {
            while (true) {
                sum += a.add(b);
            }
        }
        return sum;
    }
}
    "#;

fn not_specified(version: semver::Version, codegen: era_solc::StandardJsonInputCodegen) {
    let mut sources = BTreeMap::new();
    sources.insert("test.sol".to_owned(), LIBRARY_TEST_SOURCE.to_owned());

    let output = common::build_solidity_and_detect_missing_libraries(
        sources.clone(),
        era_solc::StandardJsonInputLibraries::default(),
        &version,
        codegen,
    )
    .expect("Test failure");
    assert!(
        output
            .contracts
            .get("test.sol")
            .expect("Always exists")
            .get("SimpleContract")
            .expect("Always exists")
            .missing_libraries
            .as_ref()
            .expect("Always exists")
            .contains("test.sol:SimpleLibrary"),
        "Missing library not detected"
    );
}

fn specified(version: semver::Version, codegen: era_solc::StandardJsonInputCodegen) {
    let mut sources = BTreeMap::new();
    sources.insert("test.sol".to_owned(), LIBRARY_TEST_SOURCE.to_owned());

    let mut libraries = BTreeMap::new();
    libraries
        .entry("test.sol".to_string())
        .or_insert_with(BTreeMap::new)
        .entry("SimpleLibrary".to_string())
        .or_insert("0x00000000000000000000000000000000DEADBEEF".to_string());
    let libraries = era_solc::StandardJsonInputLibraries::from(libraries);

    let output =
        common::build_solidity_and_detect_missing_libraries(sources, libraries, &version, codegen)
            .expect("Test failure");
    assert!(
        output
            .contracts
            .get("test.sol")
            .expect("Always exists")
            .get("SimpleContract")
            .expect("Always exists")
            .missing_libraries
            .as_ref()
            .cloned()
            .unwrap_or_default()
            .is_empty(),
        "The list of missing libraries must be empty"
    );
}
