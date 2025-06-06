//!
//! Unit tests for the optimizer.
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
    let sources =
        crate::common::read_sources(&[crate::common::TEST_SOLIDITY_CONTRACT_OPTIMIZED_PATH]);

    let build_unoptimized = crate::common::build_solidity_standard_json(
        sources.clone(),
        era_compiler_common::Libraries::default(),
        era_compiler_common::EraVMMetadataHashType::Keccak256,
        BTreeSet::new(),
        &version,
        codegen,
        era_compiler_llvm_context::OptimizerSettings::none(),
    )
    .expect("Build failure");
    let build_optimized_for_cycles = crate::common::build_solidity_standard_json(
        sources.clone(),
        era_compiler_common::Libraries::default(),
        era_compiler_common::EraVMMetadataHashType::Keccak256,
        BTreeSet::new(),
        &version,
        codegen,
        era_compiler_llvm_context::OptimizerSettings::cycles(),
    )
    .expect("Build failure");
    let build_optimized_for_size = crate::common::build_solidity_standard_json(
        sources,
        era_compiler_common::Libraries::default(),
        era_compiler_common::EraVMMetadataHashType::Keccak256,
        BTreeSet::new(),
        &version,
        codegen,
        era_compiler_llvm_context::OptimizerSettings::size(),
    )
    .expect("Build failure");

    let size_when_unoptimized = build_unoptimized
        .contracts
        .get(crate::common::TEST_SOLIDITY_CONTRACT_OPTIMIZED_PATH)
        .expect("Missing file")
        .get("Optimized")
        .expect("Missing contract")
        .evm
        .as_ref()
        .expect("Missing EVM data")
        .bytecode
        .as_ref()
        .expect("Missing bytecode")
        .object
        .len();
    let size_when_optimized_for_cycles = build_optimized_for_cycles
        .contracts
        .get(crate::common::TEST_SOLIDITY_CONTRACT_OPTIMIZED_PATH)
        .expect("Missing file")
        .get("Optimized")
        .expect("Missing contract")
        .evm
        .as_ref()
        .expect("Missing EVM data")
        .bytecode
        .as_ref()
        .expect("Missing bytecode")
        .object
        .len();
    let size_when_optimized_for_size = build_optimized_for_size
        .contracts
        .get(crate::common::TEST_SOLIDITY_CONTRACT_OPTIMIZED_PATH)
        .expect("Missing file")
        .get("Optimized")
        .expect("Missing contract")
        .evm
        .as_ref()
        .expect("Missing EVM data")
        .bytecode
        .as_ref()
        .expect("Missing bytecode")
        .object
        .len();

    assert!(
        size_when_optimized_for_cycles < size_when_unoptimized,
        "Expected cycles-optimized bytecode to be smaller than unoptimized. Optimized: {size_when_optimized_for_cycles}B, Unoptimized: {size_when_unoptimized}B",
    );
    assert!(
        size_when_optimized_for_size < size_when_unoptimized,
        "Expected size-optimized bytecode to be smaller than unoptimized. Optimized: {size_when_optimized_for_size}B, Unoptimized: {size_when_unoptimized}B",
    );
}
