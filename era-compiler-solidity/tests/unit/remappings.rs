//!
//! The Solidity compiler unit tests for remappings.
//!

use std::collections::BTreeSet;

use test_case::test_case;

use crate::common;

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
    if cfg!(target_os = "windows") && version < semver::Version::new(0, 6, 0) {
        return;
    }

    let sources = common::read_sources(&[
        common::TEST_SOLIDITY_CONTRACT_CALLER_MAIN_PATH,
        common::TEST_SOLIDITY_CONTRACT_CALLER_CALLABLE_PATH,
    ]);

    let mut remappings = BTreeSet::new();
    remappings.insert("libraries/default/=./".to_owned());

    common::build_solidity_standard_json(
        sources,
        era_solc::StandardJsonInputLibraries::default(),
        era_compiler_common::HashType::Keccak256,
        remappings,
        &version,
        codegen,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Test failure");
}
