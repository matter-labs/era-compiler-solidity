//!
//! The Solidity compiler unit tests for IR artifacts.
//!
//! The tests check if the IR artifacts are kept in the final output.
//!

use std::collections::BTreeSet;

use test_case::test_case;

use crate::common;

#[test_case(semver::Version::new(0, 4, 26))]
#[test_case(semver::Version::new(0, 5, 17))]
#[test_case(semver::Version::new(0, 6, 12))]
#[test_case(semver::Version::new(0, 7, 6))]
#[test_case(era_solc::Compiler::LAST_SUPPORTED_VERSION)]
fn evmla(version: semver::Version) {
    if cfg!(target_os = "windows") && version < semver::Version::new(0, 6, 0) {
        return;
    }

    let sources = common::read_sources(&[common::TEST_SOLIDITY_CONTRACT_PATH]);

    let build = common::build_solidity(
        sources,
        era_solc::StandardJsonInputLibraries::default(),
        era_compiler_common::HashType::Ipfs,
        BTreeSet::new(),
        &version,
        era_solc::StandardJsonInputCodegen::EVMLA,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");
    assert!(
        !build
            .contracts
            .get(common::TEST_SOLIDITY_CONTRACT_PATH)
            .expect("Always exists")
            .get("Test")
            .expect("Always exists")
            .evm
            .as_ref()
            .expect("EVM object is missing")
            .legacy_assembly
            .is_null(),
        "EVM assembly is missing",
    );
    assert!(
        build
            .contracts
            .get(common::TEST_SOLIDITY_CONTRACT_PATH)
            .expect("Always exists")
            .get("Test")
            .expect("Always exists")
            .ir_optimized
            .is_empty(),
        "Yul is present although not requested",
    );
}

#[test_case(era_solc::Compiler::LAST_SUPPORTED_VERSION)]
fn yul(version: semver::Version) {
    let sources = common::read_sources(&[common::TEST_SOLIDITY_CONTRACT_PATH]);

    let build = common::build_solidity(
        sources,
        era_solc::StandardJsonInputLibraries::default(),
        era_compiler_common::HashType::Ipfs,
        BTreeSet::new(),
        &version,
        era_solc::StandardJsonInputCodegen::Yul,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");

    assert!(
        !build
            .contracts
            .get(common::TEST_SOLIDITY_CONTRACT_PATH)
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
            .get(common::TEST_SOLIDITY_CONTRACT_PATH)
            .expect("Always exists")
            .get("Test")
            .expect("Always exists")
            .evm
            .as_ref()
            .expect("EVM object is missing")
            .legacy_assembly
            .is_null(),
        "EVM assembly is present although not requested"
    );
}
