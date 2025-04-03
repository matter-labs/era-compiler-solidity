//!
//! Unit tests for factory dependencies.
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
    if cfg!(target_os = "windows") && version < semver::Version::new(0, 6, 0) {
        return;
    }

    let sources = crate::common::read_sources(&[
        crate::common::TEST_SOLIDITY_CONTRACT_CALLER_MAIN_PATH,
        crate::common::TEST_SOLIDITY_CONTRACT_CALLER_CALLABLE_PATH,
    ]);

    let output = crate::common::build_solidity_standard_json(
        sources,
        era_compiler_common::Libraries::default(),
        era_compiler_common::HashType::Ipfs,
        BTreeSet::new(),
        &version,
        codegen,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Build failure");

    assert_eq!(
        output
            .contracts
            .get(crate::common::TEST_SOLIDITY_CONTRACT_CALLER_MAIN_PATH)
            .expect("Missing file")
            .get("Main")
            .expect("Missing contract")
            .factory_dependencies
            .len(),
        1,
        "Expected 1 factory dependency"
    );
    assert_eq!(
        output
            .contracts
            .get(crate::common::TEST_SOLIDITY_CONTRACT_CALLER_CALLABLE_PATH)
            .expect("Missing file")
            .get("Callable")
            .expect("Missing contract")
            .factory_dependencies
            .len(),
        0,
        "Expected 0 factory dependencies"
    );
}
