//!
//! CLI tests for the eponymous option.
//!

use era_compiler_common::HashType;
use era_compiler_common::Target;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(Target::EraVM, HashType::None)]
#[test_case(Target::EraVM, HashType::Keccak256)]
#[test_case(Target::EraVM, HashType::Ipfs)]
#[test_case(Target::EVM, HashType::None)]
#[test_case(Target::EVM, HashType::Keccak256)]
#[test_case(Target::EVM, HashType::Ipfs)]
fn default(target: Target, hash_type: HashType) -> anyhow::Result<()> {
    crate::common::setup()?;

    let hash_type = hash_type.to_string();
    let args = &[
        "--metadata-hash",
        hash_type.as_str(),
        "--bin",
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary:\n"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn standard_json_none(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--metadata-hash",
        "none",
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "Metadata hash mode must be specified in standard JSON input settings.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn standard_json_keccak256(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--metadata-hash",
        "keccak256",
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "Metadata hash mode must be specified in standard JSON input settings.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn standard_json_ipfs(target: Target) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--metadata-hash",
        "ipfs",
    ];

    let result = crate::cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "Metadata hash mode must be specified in standard JSON input settings.",
    ));

    Ok(())
}
