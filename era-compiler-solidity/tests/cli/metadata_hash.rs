//!
//! CLI tests for the eponymous option.
//!

use era_compiler_common::EraVMMetadataHashType;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(EraVMMetadataHashType::None.to_string())]
fn none(hash_type: String) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--metadata-hash",
        hash_type.as_str(),
        "--bin",
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains("a164"));

    Ok(())
}

#[test_case(EraVMMetadataHashType::IPFS.to_string())]
fn ipfs(hash_type: String) -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--metadata-hash",
        hash_type.as_str(),
        "--bin",
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains("a264"));

    Ok(())
}

#[test]
fn keccak256() -> anyhow::Result<()> {
    crate::common::setup()?;

    let hash_type = EraVMMetadataHashType::Keccak256.to_string();
    let args = &[
        "--metadata-hash",
        hash_type.as_str(),
        "--bin",
        crate::common::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary:\n"))
        .stderr(predicate::str::contains(
            "`keccak256` metadata hash type is deprecated. Please use `ipfs` instead.",
        ));

    Ok(())
}

#[test]
fn standard_json_keccak256() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_JSON_KECCAK256_DEPRECATED,
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "`keccak256` metadata hash type is deprecated. Please use `ipfs` instead.",
    ));

    Ok(())
}

#[test]
fn standard_json_cli_excess_arg() -> anyhow::Result<()> {
    crate::common::setup()?;

    let args = &[
        "--standard-json",
        crate::common::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--metadata-hash",
        "ipfs",
    ];

    let result = crate::cli::execute_zksolc(args)?;
    result.success().stdout(predicate::str::contains(
        "Metadata hash mode must be specified in standard JSON input settings.",
    ));

    Ok(())
}
