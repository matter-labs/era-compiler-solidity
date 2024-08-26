#![cfg(test)]

pub mod cli_tests;
pub mod common;

use predicates::prelude::*;

#[test]
fn run_zksolc_with_metadata_hash_default() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[
        cli_tests::TEST_SOLIDITY_CONTRACT_PATH,
        "--metadata-hash=none",
    ];

    let result = cli_tests::execute_zksolc(args)?;
    let zksolc_result = result
        .success()
        .stderr(predicate::str::contains("Compiler run successful"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli_tests::execute_solc(args)?;
    solc_result.code(zksolc_result);

    Ok(())
}

#[test]
fn run_zksolc_with_metadata_hash_no_arg() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &[cli_tests::TEST_SOLIDITY_CONTRACT_PATH, "--metadata-hash"];

    let result = cli_tests::execute_zksolc(args)?;
    let zksolc_result = result
        .failure()
        .stderr(predicate::str::contains(
            "requires a value but none was supplied",
        ))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli_tests::execute_solc(args)?;
    solc_result.code(zksolc_result);

    Ok(())
}

#[test]
fn run_zksolc_with_metadata_hash_no_input_file() -> anyhow::Result<()> {
    let _ = common::setup();
    let args = &["--metadata-hash=none"];

    let result = cli_tests::execute_zksolc(args)?;
    let zksolc_result = result
        .failure()
        .stderr(predicate::str::contains("No input sources specified"))
        .get_output()
        .status
        .code()
        .expect("No exit code.");

    let solc_result = cli_tests::execute_solc(args)?;
    solc_result.code(zksolc_result);

    Ok(())
}
