//!
//! Unit tests for libraries.
//!

use std::collections::BTreeSet;

use test_case::test_case;

#[test_case(
    semver::Version::new(0, 4, 26),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 5, 17),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 6, 12),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 7, 6),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::Yul
)]
fn not_specified(version: semver::Version, codegen: era_solc::StandardJsonInputCodegen) {
    if cfg!(target_os = "windows") && version < semver::Version::new(0, 6, 0) {
        return;
    }

    let sources =
        crate::common::read_sources(&[crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH]);

    let output = crate::common::build_solidity_standard_json(
        sources,
        era_solc::StandardJsonInputLibraries::default(),
        era_compiler_common::HashType::Ipfs,
        BTreeSet::new(),
        &version,
        codegen,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");

    assert!(
        output
            .contracts
            .get(crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH)
            .expect("Always exists")
            .get("SimpleContract")
            .expect("Always exists")
            .missing_libraries
            .contains(
                format!(
                    "{}:SimpleLibrary",
                    crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH
                )
                .as_str()
            ),
        "Missing library not detected"
    );
}

#[test_case(
    semver::Version::new(0, 4, 26),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 5, 17),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 6, 12),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    semver::Version::new(0, 7, 6),
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::EVMLA
)]
#[test_case(
    era_solc::Compiler::LAST_SUPPORTED_VERSION,
    era_solc::StandardJsonInputCodegen::Yul
)]
fn specified(version: semver::Version, codegen: era_solc::StandardJsonInputCodegen) {
    if cfg!(target_os = "windows") && version < semver::Version::new(0, 6, 0) {
        return;
    }

    let sources =
        crate::common::read_sources(&[crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH]);

    let mut libraries = era_solc::StandardJsonInputLibraries::default();
    libraries
        .as_inner_mut()
        .entry(crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH.to_string())
        .or_default()
        .entry("SimpleLibrary".to_string())
        .or_insert("0x00000000000000000000000000000000DEADBEEF".to_string());

    let output = crate::common::build_solidity_standard_json(
        sources,
        libraries,
        era_compiler_common::HashType::Ipfs,
        BTreeSet::new(),
        &version,
        codegen,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");
    assert!(
        output
            .contracts
            .get(crate::common::TEST_SOLIDITY_CONTRACT_SIMPLE_CONTRACT_PATH)
            .expect("Always exists")
            .get("SimpleContract")
            .expect("Always exists")
            .missing_libraries
            .is_empty(),
        "The list of missing libraries must be empty"
    );
}
