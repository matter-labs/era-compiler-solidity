//!
//! The Solidity compiler unit tests for libraries.
//!

#![cfg(test)]

pub mod common;

use std::collections::BTreeMap;

use era_compiler_solidity::solc::pipeline::Pipeline as SolcPipeline;
use era_compiler_solidity::solc::Compiler as SolcCompiler;

#[test]
#[cfg_attr(target_os = "windows", ignore)]
fn not_specified_04_evmla() {
    not_specified(semver::Version::new(0, 4, 26), SolcPipeline::EVMLA);
}
#[test]
#[cfg_attr(target_os = "windows", ignore)]
fn not_specified_05_evmla() {
    not_specified(semver::Version::new(0, 5, 17), SolcPipeline::EVMLA);
}
#[test]
fn not_specified_06_evmla() {
    not_specified(semver::Version::new(0, 6, 12), SolcPipeline::EVMLA);
}
#[test]
fn not_specified_07_evmla() {
    not_specified(semver::Version::new(0, 7, 6), SolcPipeline::EVMLA);
}
#[test]
fn not_specified_08_evmla() {
    not_specified(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::EVMLA);
}
#[test]
fn not_specified_08_yul() {
    not_specified(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::Yul);
}

#[test]
#[cfg_attr(target_os = "windows", ignore)]
fn specified_04_evmla() {
    specified(semver::Version::new(0, 4, 26), SolcPipeline::EVMLA);
}
#[test]
#[cfg_attr(target_os = "windows", ignore)]
fn specified_05_evmla() {
    specified(semver::Version::new(0, 5, 17), SolcPipeline::EVMLA);
}
#[test]
fn specified_06_evmla() {
    specified(semver::Version::new(0, 6, 12), SolcPipeline::EVMLA);
}
#[test]
fn specified_07_evmla() {
    specified(semver::Version::new(0, 7, 6), SolcPipeline::EVMLA);
}
#[test]
fn specified_08_evmla() {
    specified(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::EVMLA);
}
#[test]
fn specified_08_yul() {
    specified(SolcCompiler::LAST_SUPPORTED_VERSION, SolcPipeline::Yul);
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

fn not_specified(version: semver::Version, pipeline: SolcPipeline) {
    let mut sources = BTreeMap::new();
    sources.insert("test.sol".to_owned(), LIBRARY_TEST_SOURCE.to_owned());

    let output = common::build_solidity_and_detect_missing_libraries(
        sources.clone(),
        BTreeMap::new(),
        &version,
        pipeline,
    )
    .expect("Test failure");
    assert!(
        output
            .contracts
            .as_ref()
            .expect("Always exists")
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

fn specified(version: semver::Version, pipeline: SolcPipeline) {
    let mut sources = BTreeMap::new();
    sources.insert("test.sol".to_owned(), LIBRARY_TEST_SOURCE.to_owned());

    let mut libraries = BTreeMap::new();
    libraries
        .entry("test.sol".to_string())
        .or_insert_with(BTreeMap::new)
        .entry("SimpleLibrary".to_string())
        .or_insert("0x00000000000000000000000000000000DEADBEEF".to_string());

    let output = common::build_solidity_and_detect_missing_libraries(
        sources.clone(),
        libraries.clone(),
        &version,
        pipeline,
    )
    .expect("Test failure");
    assert!(
        output
            .contracts
            .as_ref()
            .expect("Always exists")
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
