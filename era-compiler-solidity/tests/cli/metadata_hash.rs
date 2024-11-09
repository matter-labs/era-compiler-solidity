use crate::{cli, common};
use era_compiler_common::Target;
use predicates::prelude::*;
use test_case::test_case;

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_metadata_hash_default(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[cli::TEST_SOLIDITY_CONTRACT_PATH, "--metadata-hash", "none"];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let zksolc_result = result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args)?;
    solc_result.code(zksolc_result);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_metadata_hash_no_argument(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[cli::TEST_SOLIDITY_CONTRACT_PATH, "--metadata-hash"];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let zksolc_result = result
        .failure()
        .stderr(predicate::str::contains(
            "requires a value but none was supplied",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args)?;
    solc_result.code(zksolc_result);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_metadata_hash_no_input_file(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &["--metadata-hash", "none"];

    let result = cli::execute_zksolc_with_target(args, target)?;
    let zksolc_result = result
        .failure()
        .stderr(predicate::str::contains("No input sources specified"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli::execute_solc(args)?;
    solc_result.code(zksolc_result);

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_metadata_hash_none(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--metadata-hash",
        "none",
        "--bin",
        cli::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary:\n"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_metadata_hash_keccak256(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--metadata-hash",
        "keccak256",
        "--bin",
        cli::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary:\n"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_metadata_hash_ipfs(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--metadata-hash",
        "ipfs",
        "--bin",
        cli::TEST_SOLIDITY_CONTRACT_PATH,
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result
        .success()
        .stdout(predicate::str::contains("Binary:\n"));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_metadata_hash_none_standard_json_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--metadata-hash",
        "none",
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "Metadata hash mode must be specified in standard JSON input settings.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_metadata_hash_keccak256_standard_json_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--metadata-hash",
        "keccak256",
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "Metadata hash mode must be specified in standard JSON input settings.",
    ));

    Ok(())
}

#[test_case(Target::EraVM)]
#[test_case(Target::EVM)]
fn with_metadata_hash_ipfs_standard_json_mode(target: Target) -> anyhow::Result<()> {
    common::setup()?;

    let args = &[
        "--standard-json",
        cli::TEST_SOLIDITY_STANDARD_JSON_SOLC_PATH,
        "--metadata-hash",
        "ipfs",
    ];

    let result = cli::execute_zksolc_with_target(args, target)?;
    result.success().stdout(predicate::str::contains(
        "Metadata hash mode must be specified in standard JSON input settings.",
    ));

    Ok(())
}
