//!
//! Unit tests for remappings.
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
fn default(version: semver::Version, codegen: era_solc::StandardJsonInputCodegen) {
    let sources = crate::common::read_sources(&[
        crate::common::TEST_SOLIDITY_CONTRACT_CALLER_MAIN_PATH,
        crate::common::TEST_SOLIDITY_CONTRACT_CALLER_CALLABLE_PATH,
    ]);

    let mut remappings = BTreeSet::new();
    remappings.insert("libraries/default/=./".to_owned());

    crate::common::build_solidity_standard_json(
        sources,
        era_compiler_common::Libraries::default(),
        era_compiler_common::MetadataHashType::Keccak256,
        remappings,
        &version,
        codegen,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");
}
