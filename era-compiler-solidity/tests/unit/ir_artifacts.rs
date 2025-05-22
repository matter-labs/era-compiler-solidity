//!
//! Unit tests for IR artifacts.
//!
//! The tests check if the IR artifacts are kept in the final output.
//!

use std::collections::BTreeSet;

use test_case::test_case;

#[test_case(semver::Version::new(0, 4, 26))]
#[test_case(semver::Version::new(0, 5, 17))]
#[test_case(semver::Version::new(0, 6, 12))]
#[test_case(semver::Version::new(0, 7, 6))]
#[test_case(era_solc::Compiler::LAST_SUPPORTED_VERSION)]
fn evmla(version: semver::Version) {
    let sources = crate::common::read_sources(&[crate::common::TEST_SOLIDITY_CONTRACT_PATH]);

    let build = crate::common::build_solidity_standard_json(
        sources,
        era_compiler_common::Libraries::default(),
        era_compiler_common::EraVMMetadataHashType::IPFS,
        BTreeSet::new(),
        &version,
        era_solc::StandardJsonInputCodegen::EVMLA,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");
    assert!(
        !build
            .contracts
            .get(crate::common::TEST_SOLIDITY_CONTRACT_PATH)
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
            .get(crate::common::TEST_SOLIDITY_CONTRACT_PATH)
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
    let sources = crate::common::read_sources(&[crate::common::TEST_SOLIDITY_CONTRACT_PATH]);

    let build = crate::common::build_solidity_standard_json(
        sources,
        era_compiler_common::Libraries::default(),
        era_compiler_common::EraVMMetadataHashType::IPFS,
        BTreeSet::new(),
        &version,
        era_solc::StandardJsonInputCodegen::Yul,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");

    assert!(
        !build
            .contracts
            .get(crate::common::TEST_SOLIDITY_CONTRACT_PATH)
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
            .get(crate::common::TEST_SOLIDITY_CONTRACT_PATH)
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

#[test_case(era_solc::Compiler::LAST_SUPPORTED_VERSION)]
fn yul_empty_solidity_interface(version: semver::Version) {
    let sources = crate::common::read_sources(&[
        crate::common::TEST_SOLIDITY_CONTRACT_INTERFACE_EMPTY_YUL_PATH,
    ]);

    let build = crate::common::build_solidity_standard_json(
        sources,
        era_compiler_common::Libraries::default(),
        era_compiler_common::EraVMMetadataHashType::IPFS,
        BTreeSet::new(),
        &version,
        era_solc::StandardJsonInputCodegen::Yul,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");

    assert_eq!(build.contracts.len(), 1, "More than one Yul object present");
}
